"""Independent completeness, owner-baseline and replay audit; no timing inference."""
import hashlib
import itertools
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s08-stream-ownership-gate'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    f=json.loads((OUT/'freeze.json').read_text())
    for name,h in f['sources'].items():assert sha(OUT/'source'/name)==h==sha(ROOT/name)
    for name,h in f['binary'].items():assert sha(Path(name))==h
    modes=['direct','compact-live','scan','sealed','dependencies','templates','lowered']
    configs=[[m,f,n,r,k,0] for f,n,r,k,m in itertools.product(['repeated','distinct','aliases'],[16,64],[0,1],['0','4','all'],modes)]
    cancels=[[m,'aliases',64,1,k,1] for k,m in itertools.product(['0','4','all'],modes)]
    assert f['configs']==configs and f['cancel']==cancels
    rows=json.loads((OUT/'summary.json').read_text());assert len(rows)==252
    groups={};hashes={};values={}
    for kind,cs in [('complete',configs),('cancel',cancels)]:
        for i,c in enumerate(cs):
            for rep in range(2):
                path=OUT/f'{kind}-{rep}-{i:03}.json';raw=json.loads(path.read_text())
                assert raw['returncode']==0 and not raw.get('timeout')
                assert raw['command']==list(f['binary'])+list(map(str,c))
                v=json.loads(raw['stdout'].splitlines()[-1]);hashes[path.name]=sha(path)
                assert v['mode']==c[0] and v['family']==c[1] and v['depth']==c[2]
                assert v['resource']==bool(c[3]) and v['keep']==c[4] and v['cancel']==bool(c[5])
                assert v['meter'] and not v['counters']
                assert v['restored']['live_start']==v['restored']['live_end']
                assert v['snapshots'][0]['label']=='prepared'
                for q in range(2):
                    ss=[s for s in v['snapshots'] if s['query']==q and s['label']!='prepared']
                    count=4 if kind=='cancel' and q==0 else c[2]+q+1
                    labels=['setup']+['delivery']*sum(x<=count for x in [1,4,16,64])+['cancelled' if kind=='cancel' and q==0 else 'exhausted','consumer-released','engine-disposed']
                    assert [s['label'] for s in ss]==labels
                    assert [s['answers'] for s in ss if s['label']=='delivery']==[x for x in [1,4,16,64] if x<=count]
                    assert all(s['answers']==count for s in ss[-3:])
                    assert len({s['baseline'] for s in ss})==1
                    assert ss[-1]['memory']['live_end']==ss[-1]['baseline']
                    keep=count if c[4]=='all' else min(count,int(c[4]))
                    assert ss[-3]['retained']==keep and ss[-2]['retained']==0
                    assert ss[-3]['memory']['live_end']>=ss[-2]['memory']['live_end']
                    if rep==0 and kind=='complete':
                        key=tuple(c[:4])+ (q,)
                        groups.setdefault(key,[]).append(ss[-2]['memory']['live_end']-ss[-2]['baseline'])
                if rep:assert v==values[kind,i]
                else:values[kind,i]=v
            if kind=='complete':
                assert rows[i]['config']==c
                expected=[dict(query=s['query'],label=s['label'],answers=s['answers'],retained=s['retained'],live_growth=s['memory']['live_end']-s['baseline'],requested_bytes=s['memory']['requested_bytes']) for s in v['snapshots']]
                assert rows[i]['snapshots']==expected
    assert len(groups)==168
    assert all(len(v)==3 and len(set(v))==1 for v in groups.values())
    (OUT/'audit.json').write_text(json.dumps({'processes':len(hashes),'exact_replays':273,
        'consumer_independent_engine_snapshots':168,'source_files':len(f['sources']),
        'all_disposal_baselines_restored':True,'receipt_hashes':hashes},indent=2)+'\n')
    print('546 receipts,273 exact replays,168 consumer-independent engine snapshots verified')

if __name__=='__main__':main()
