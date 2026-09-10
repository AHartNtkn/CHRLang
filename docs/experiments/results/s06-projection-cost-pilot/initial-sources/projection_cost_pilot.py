"""Run prospective projection controls, sizing and the bounded cost pilot."""
from pathlib import Path
import hashlib,json,os,platform,random,resource,subprocess,sys,time
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-projection-cost-pilot'
MODES=['ascending','descending','greedy','enumerate','separable','structural']
PRIMARY=ROOT/'target/s06-projection-cost-primary/release/examples/projection_cost'
METER=ROOT/'target/s06-projection-cost-meter/release/examples/projection_cost'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def cells():
    return [(m,f,n,q,e,c) for f in ['independent','star','dense'] for n in [4,10] for q in [1,4] for e in ['weighted','expanded'] for c in [0,8] for m in MODES if not(m=='separable' and f=='star')]
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def run(command,path):
    assert not path.exists(),path
    start=time.monotonic()
    try:
        p=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=bounds)
        r=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        r=dict(command=command,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    r['wall_seconds']=time.monotonic()-start
    path.write_text(json.dumps(r,indent=2)+'\n')
    assert r.get('returncode')==0 and not r['stderr'],path
    return r
if __name__=='__main__':
    action=sys.argv[1];assert action in ['qualify','pilot']
    manifest=RAW/'manifest.json'
    if action=='qualify':
        artifacts=[json.loads(s) for s in (RAW/'control-build.jsonl').read_text().splitlines()]
        tests=[Path(r['executable']) for r in artifacts if r.get('reason')=='compiler-artifact' and r.get('executable') and r['target']['name']=='projection_cost_controls'];assert len(tests)==1
        sources=[Path(__file__),ROOT/'research/chr-structural/experiments/audit_projection_cost.py',ROOT/'research/chr-structural/examples/projection_cost.rs',ROOT/'research/chr-structural/examples/support/projection_cost.rs',ROOT/'research/chr-structural/tests/projection_cost_controls.rs',ROOT/'research/chr-structural/src/projection.rs',ROOT/'research/chr-structural/src/finite.rs',ROOT/'research/chr-structural/src/lib.rs',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'research/chr-structural/Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S06-projection-cost-pilot.md',PRIMARY,METER]+tests
        assert not manifest.exists()
        manifest.write_text(json.dumps(dict(cells=cells(),primary=str(PRIMARY.relative_to(ROOT)),meter=str(METER.relative_to(ROOT)),control=str(tests[0].relative_to(ROOT)),sha256={str(p.relative_to(ROOT)):digest(p) for p in sources},host=dict(platform=platform.platform(),affinity=sorted(os.sched_getaffinity(0)),rustc=subprocess.check_output(['rustc','--version'],text=True).strip())),indent=2)+'\n')
        run([str(tests[0]),'--test-threads=1','--nocapture'],RAW/'controls.json')
        for i,cell in enumerate(c for c in cells() if c[2:]==(10,4,'expanded',0)):
            r=run([str(PRIMARY),*map(str,cell)],RAW/f'sizing-{i:02}.json')
            assert r['wall_seconds']<10,'sizing needs review before pilot'
        print('Qualified controls and 17 sizing processes; all sizing processes below 10 seconds.',flush=True)
    else:
        m=json.loads(manifest.read_text())
        for p,h in m['sha256'].items():assert digest(ROOT/p)==h,p
        assert len(list(RAW.glob('sizing-*.json')))==17
        for path in RAW.glob('sizing-*.json'):
            r=json.loads(path.read_text());assert r['returncode']==0 and r['wall_seconds']<10
        tasks=[(kind,repeat,cell) for kind,count in [('primary',5),('meter',2)] for repeat in range(count) for cell in cells()]
        assert len(tasks)==1904
        random.Random(607031).shuffle(tasks)
        schedule=RAW/'schedule.json';assert not schedule.exists();schedule.write_text(json.dumps(tasks)+'\n')
        for i,(kind,repeat,cell) in enumerate(tasks):
            binary=PRIMARY if kind=='primary' else METER
            run([str(binary),*map(str,cell)],RAW/f'run-{i:04}.json')
            if i%100==99:print(i+1,'/ 1904 terminal processes',flush=True)
        print('Pilot complete: 1904 terminal processes.',flush=True)
