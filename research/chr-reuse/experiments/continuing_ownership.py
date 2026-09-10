"""Prospective continuing-emission ownership, with auditable bounded preflights."""
from pathlib import Path
import hashlib,itertools,json,resource,subprocess,time,sys
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s08-continuing-ownership'
BINARY=ROOT/'target/s08-continuing-ownership/release/examples/continuing_ownership'
MODES=['direct','compact-live','scan','sealed','conditional','dependencies','templates','dependencies-reclaim','templates-reclaim']
CONFIGS=list(itertools.product(MODES,[32,128,512],['0','4','all'],[1,4]))
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def validate(row,config):
    assert [row[k] for k in ['mode','demand','keep','capacity']]==list(config)
    mode,n,keep,cap=config;ss=row['snapshots'];base=row['baseline']
    assert row['blocked']>0
    prefixes=[s for s in ss if s['label']=='prefix']
    assert [s['delivered'] for s in prefixes]==[i for i in [1,8,32,128,512] if i<=n]
    assert [s['label'] for s in ss]==['prepared','setup']+['prefix']*len(prefixes)+['before-engine-drop','engine-dropped','consumer-released','prepared-dropped']
    assert ss[-1]['memory']['live_end']==base
    assert ss[-2]['memory']['live_end']==ss[0]['memory']['live_end']
    for s in ss:
        m=s['memory'];assert m['live_start']==base
        assert m['peak_live']>=m['live_end'] and m['peak_live']>=base
        assert s['queued']<=cap
        if s['label'] in ['prefix','before-engine-drop','engine-dropped']:
            assert s['retained']==min(s['delivered'],n if keep=='all' else int(keep))
    for a,b in zip(ss,ss[1:]):
        assert a['memory']['requested_bytes']<=b['memory']['requested_bytes']
        assert a['memory']['peak_live']<=b['memory']['peak_live']
    assert ss[-3]['queued']==0
    if keep=='0':assert ss[-3]['memory']['live_end']==ss[0]['memory']['live_end']
    if mode.endswith('-reclaim'):assert row['removed_results']>0
    else:assert row['removed_results']==0

def invoke(config,path,deadline):
    remaining=deadline-time.monotonic();assert remaining>0,'launcher bound'
    command=[str(BINARY),*map(str,config)]
    try:
        p=subprocess.run(command,capture_output=True,text=True,timeout=min(60,remaining),preexec_fn=bounds)
        receipt=dict(config=config,command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        receipt=dict(config=config,command=command,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))
        path.write_text(json.dumps(receipt)+'\n');raise
    path.write_text(json.dumps(receipt)+'\n')
    assert p.returncode==0 and not p.stderr,receipt
    row=json.loads(p.stdout);validate(row,config);return row

def analyze():
    manifest=json.loads((RAW/'manifest.json').read_text())
    for f,h in manifest['sha256'].items():assert digest(ROOT/f)==h,f
    rows=[]
    for cell,c in enumerate(CONFIGS):
        pair=[]
        for repeat in range(2):
            receipt=json.loads((RAW/f'run-{cell:03}-{repeat}.json').read_text())
            assert receipt['config']==list(c) and receipt['returncode']==0 and not receipt['stderr']
            r=json.loads(receipt['stdout']);validate(r,c);pair.append(r)
        assert pair[0]==pair[1],c
        rows.append(pair[0])
    for mode,n,cap in itertools.product(MODES,[32,128,512],[1,4]):
        rs=[r for r in rows if (r['mode'],r['demand'],r['capacity'])==(mode,n,cap)]
        deltas=[r['snapshots'][-4]['memory']['live_end']-r['snapshots'][-3]['memory']['live_end'] for r in rs]
        assert len(rs)==3 and len(set(deltas))==1,(mode,n,cap,deltas)
    audit=dict(configurations=162,paired_processes=324,preflights=9,rows=rows,
               receipts_sha256={p.name:digest(p) for p in sorted(RAW.glob('run-*.json'))})
    (RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n')
    print('162 exact allocation pairs, 324 processes: counts, trajectories and final owners validated.')

if __name__=='__main__':
    if sys.argv[1:]==['--analyze']:analyze();raise SystemExit
    assert not sys.argv[1:] and not (RAW/'manifest.json').exists()
    paths=[Path(__file__),BINARY,ROOT/'research/chr-reuse/examples/continuing_ownership.rs',ROOT/'research/chr-reuse/tests/consumer_pressure.rs',ROOT/'research/chr-reuse/examples/support/stream_run.rs',ROOT/'research/chr-reuse/examples/support/stream_source.rs',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'docs/experiments/registrations/S08-continuing-ownership.md',ROOT/'Cargo.lock',ROOT/'research/chr-reuse/Cargo.toml']
    for crate in ['chr-reuse','chr-direct-choice','chr-direct-conditional','chr-compiled','chr-observe','chr-persistent']:
        paths.extend((ROOT/'research'/crate/'src').rglob('*.rs'))
    manifest=dict(configurations=CONFIGS,preflights=[(m,512,'all',4) for m in MODES],sha256={str(p.relative_to(ROOT)):digest(p) for p in paths})
    (RAW/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    check=subprocess.run([BINARY,'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
    (RAW/'meter-check.log').write_text(check.stdout+check.stderr);assert check.returncode==0
    deadline=time.monotonic()+600
    for i,m in enumerate(MODES):
        r=invoke((m,512,'all',4),RAW/f'preflight-{i}.json',deadline)
        print(m,'preflight: peak',r['snapshots'][-1]['memory']['peak_live']-r['baseline'],flush=True)
    for i,c in enumerate(CONFIGS):
        for repeat in range(2):invoke(c,RAW/f'run-{i:03}-{repeat}.json',deadline)
        if (i+1)%18==0:print(i+1,'/162 pairs',flush=True)
    analyze()
