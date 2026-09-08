from pathlib import Path
import hashlib,json,statistics,subprocess
ROOT=Path(__file__).resolve().parents[4]
OUT=Path(__file__).resolve().parent
def classify(rs):
    m=statistics.median(rs)
    if m<=.9 and max(rs)<1:return 'benefit'
    if m>=1.1 and min(rs)>1:return 'loss'
    return 'unresolved'
def main():
    freeze=json.loads((OUT/'freeze.json').read_text())
    for name,h in freeze['source'].items():
        content=subprocess.check_output(['git','show',freeze['source_commit']+':'+name],cwd=ROOT) if 'source_commit' in freeze else (ROOT/name).read_bytes()
        assert hashlib.sha256(content).hexdigest()==h,name
    # Historical receipts validate archival source; binary hashes identify the measured artifacts.
    if 'source_commit' not in freeze:
        for name,h in freeze['binaries'].items():assert hashlib.sha256((ROOT/'target/release/examples'/name).read_bytes()).hexdigest()==h,name
    data={};warmups=0
    for p in OUT.glob('*.json'):
        r=json.loads(p.read_text())
        if 'cell' not in r:continue
        assert r['exit_code']==0 and not r['cutoff'],p
        rows=[json.loads(line) for line in r['stdout'].splitlines()]
        family,depth,queries,mode=r['cell']
        assert [rows[0][k] for k in ['family','depth','queries']]==[family,depth,queries]
        if mode in ['inline','workers4']:assert rows[0]['workers']==(0 if mode=='inline' else 4)
        else:assert rows[0]['mode']==mode
        if r['allocation']:
            a=rows[-1].get('allocation',rows[-1]);assert a['live_start']==a['live_end']
            fields=dict(bytes=a['requested_bytes'],calls=a['allocation_calls'],frees=a['deallocation_calls'],peak=a['peak_live']-a['live_start'])
        else:
            if r['block']<0:warmups+=1;continue
            h=next(x for x in rows if 'lifecycle_ns' in x);qs=[x for x in rows if 'query' in x]
            assert len(qs)==queries and [q['query'] for q in qs]==list(range(queries))
            assert h['lifecycle_ns']==h['prepare_ns']+h['shutdown_ns']+h.get('runtime_drop_ns',0)+sum(q['setup_ns']+q['execute_observe_ns']+q.get('close_ns',0)+q['dispose_ns'] for q in qs)
            assert all(0<=q['first_observation_ns']<=q['execute_observe_ns'] for q in qs)
            fields=dict(lifecycle=h['lifecycle_ns'],cpu=r['process_cpu_seconds'],first=qs[0]['first_observation_ns'],prepare=h['prepare_ns'],
                        setup=sum(q['setup_ns'] for q in qs),execute=sum(q['execute_observe_ns'] for q in qs))
        data[(tuple(r['cell']),r['allocation'],r['block'])]=fields
    assert len(data)==400 and warmups==40
    summary=[]
    for family,depth,queries in sorted({c[0][:3] for c in data}):
        for mode in ['inline','workers4','specialized','contracted']:
            row=dict(cell=[family,depth,queries,mode])
            for control in ['inline','workers4','specialized','contracted']:
                if mode==control:continue
                stats={}
                for alloc,n in [(False,7),(True,3)]:
                    for field in data[((family,depth,queries,mode),alloc,0)]:
                        vs=[data[((family,depth,queries,mode),alloc,b)][field] for b in range(n)]
                        bs=[data[((family,depth,queries,control),alloc,b)][field] for b in range(n)]
                        rs=[a/b for a,b in zip(vs,bs)]
                        stats[field]=dict(values=vs,ratios=rs,median=statistics.median(vs),ratio_median=statistics.median(rs),
                                          classification='descriptive' if alloc else classify(rs))
                row[control]=stats
            summary.append(row)
    (OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    for mode,control in [('contracted','workers4'),('specialized','workers4'),('contracted','inline'),('contracted','specialized')]:
        rs=[r for r in summary if r['cell'][-1]==mode]
        print(mode,'versus',control,{c:sum(r[control]['lifecycle']['classification']==c for r in rs) for c in ['benefit','loss','unresolved']})
    for r in summary:
        if r['cell'][-1]=='contracted':print(r['cell'],[(k,round(r['workers4'][k]['ratio_median'],3)) for k in ['lifecycle','cpu','bytes','peak']])
if __name__=='__main__':main()
