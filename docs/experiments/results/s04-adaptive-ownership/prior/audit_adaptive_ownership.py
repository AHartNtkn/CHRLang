"""Verify frozen regression coverage and independently released owner lifetimes."""
from pathlib import Path
import collections,hashlib,itertools,json,re
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s04-adaptive-ownership'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
if __name__=='__main__':
    m=json.loads((RAW/'manifest.json').read_text())
    for p,h in m['sha256'].items():assert digest(ROOT/p)==h,p
    totals=collections.Counter();tests={}
    for build,test,binary in m['binaries']:
        r=json.loads((RAW/f'{build}-{test}.json').read_text());assert r['returncode']==0 and not r['stderr']
        assert r['command']==[str(ROOT/binary),'--test-threads=1']+(['--nocapture'] if test=='eager_schedule' else [])
        result=re.search(r'test result: ok. (\d+) passed; 0 failed; 0 ignored;',r['stdout']);assert result
        count=int(result[1]);totals[build]+=count
        names=re.findall(r'^test (\S+) \.\.\.',r['stdout'],re.M);assert len(names)==count
        tests[build,test]=names
        if test=='eager_schedule':
            assert count==2
            assert ('eager=88 scheduled=120 schedule=32' if build=='primary' else 'eager=152 scheduled=184 schedule=32') in r['stdout']
    assert set(totals)=={'primary','diagnostic'}
    assert len(tests['primary','adaptive_reunion'])==2 and len(tests['diagnostic','adaptive_reunion'])==3
    jobs=json.loads((RAW/'schedule.json').read_text());assert len(jobs)==576
    seen=set();pairs=collections.defaultdict(list)
    for i,(repeat,cell) in enumerate(jobs):
        key=tuple(cell);assert (repeat,key) not in seen;seen.add((repeat,key))
        r=json.loads((RAW/f'owner-{i:03}.json').read_text());assert r['returncode']==0 and not r['stderr']
        assert r['command']==[str(ROOT/m['owner']),*map(str,cell)]
        lines=r['stdout'].splitlines();assert len([s for s in lines if s.startswith('preflight query=')])==4
        data=[json.loads(s) for s in lines if s.startswith('{')];summary=data[0];rows=data[1:]
        mode,family,depth,retain,cancel=cell
        assert summary['counts']==([1]*4 if cancel else [16]*4)
        wanted=0 if retain=='0' else min(sum(summary['counts']),4) if retain=='4' else sum(summary['counts'])
        assert summary['retained']==wanted and summary['unreleased_bytes']==0
        assert summary['requested_bytes']==sum(row['memory']['requested_bytes'] for row in rows)
        assert rows[0]['phase']=='prepare' and rows[-2]['phase']=='prepared_dispose' and rows[-1]['phase']=='consumer_dispose'
        baseline=rows[0]['memory']['live_start'];assert rows[-1]['memory']['live_end']==baseline
        assert rows[-2]['memory']['live_end']-baseline==summary['consumer_bytes']
        for phase in ['input','setup','engine_dispose','input_dispose']:
            assert [row['query'] for row in rows if row['phase']==phase]==list(range(4))
        if retain=='0':
            assert summary['consumer_bytes']==0
            assert all(row['memory']['live_end']==rows[0]['memory']['live_end'] for row in rows if row['phase']=='input_dispose')
        if mode in ['eager','scheduled']:assert summary['engine_bytes']==(88 if mode=='eager' else 120)
        pairs[key].append(data)
    modes=['copy','reunion','eager','scheduled','fixed1','fixed8','backoff1','backoff8']
    assert seen=={(r,c) for r in range(2) for c in itertools.product(modes,['plain','history','late'],[0,4],['0','4','all'],[0,1])}
    cells=[]
    for key,pair in pairs.items():
        assert pair[0]==pair[1],key
        cells.append(dict(cell=key,summary=pair[0][0]))
    for mode,family,depth,retain,cancel in pairs:
        if mode=='eager':
            eager=pairs[(mode,family,depth,retain,cancel)][0]
            scheduled=pairs[('scheduled',family,depth,retain,cancel)][0]
            assert eager[1:]==scheduled[1:],'paired eager heap differs'
    (RAW/'audit.json').write_text(json.dumps(dict(status='passed',test_totals=totals,test_processes=len(m['binaries']),ownership_processes=576,cells=cells),indent=2)+'\n')
    print('Validated regression totals',dict(totals),'and 576 exact paired owner diagnostics; eager/scheduled heap traces match.')
