"""Independent receipt, owner and primary-cost audit for stream sizing."""
import hashlib,itertools,json,random
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s08-reclamation-lifecycle-sizing'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def strip(v):
    if isinstance(v,dict):return {k:strip(x) for k,x in v.items() if k not in ['ns','first_answer_ns']}
    if isinstance(v,list):return [strip(x) for x in v]
    return v
def main():
    f=json.loads((OUT/'freeze.json').read_text())
    configs=[list(c) for c in itertools.product(['repeated','distinct','aliases'],[16,64],[0,4],[0,8],[0,1],['0','4','all'])]
    configs=[c+[0] for c in configs];assert f['configs']==configs
    modes=['direct','compact-live','scan','sealed','dependencies','dependencies-reclaim','dependencies-periodic','templates','templates-reclaim','templates-periodic','lowered'];assert f['modes']==modes
    order=list(range(1584));random.Random(7911).shuffle(order);assert json.loads((OUT/'ordinary-order.json').read_text())==order
    for p,h in f['sources'].items():assert sha(ROOT/p)==h==sha(OUT/'source'/p)
    for p,h in f['binaries'].items():assert sha(Path(p))==h
    data={};hashes={}
    for kind,size in [('allocation',3168),('ordinary',1584),('cancel',33),('cancel-meter',33)]:
        for index in range(size):
            if kind.startswith('cancel'):
                k,j=divmod(index,11);c=['aliases',64,4,8,1,['0','4','all'][k],1]
            else:i,j=divmod(index%1584,11);c=configs[i]
            path=OUT/f'{kind}-{index:04}.json';raw=json.loads(path.read_text())
            binary=Path('/tmp/chr-stream-cost-a31f14574')/('meter' if kind in ['allocation','cancel-meter'] else 'ordinary')
            assert raw['command']==[str(binary),modes[j]]+list(map(str,c))
            assert raw['returncode']==0 and not raw.get('timeout')
            v=json.loads(raw['stdout'].splitlines()[-1]);assert v['mode']==modes[j]
            assert [v['family'],v['depth'],v['work'],v['payload'],int(v['resource']),v['keep'],int(v['cancel'])]==c
            assert not v['counters'] and v['meter']==(binary.name=='meter')
            assert len(v['samples'])==2
            for q,s in enumerate(v['samples']):
                count=4 if c[6] and q==0 else c[1]+q+1
                assert s['answers']==count and s['depth']==c[1]+q
                assert s['complete']==(not c[6] or q==1)
                assert 0<=s['first_answer_ns']<=s['execute_observe']['ns']
                if v['meter']:
                    ss=[x for x in v['snapshots'] if x['query']==q]
                    expected=['setup']+['delivery']*sum(x<=count for x in [1,4,16,64])+['exhausted' if s['complete'] else 'cancelled','consumer-released','engine-disposed']
                    assert [x['label'] for x in ss]==expected
                    assert len({x['baseline'] for x in ss})==1
                    assert ss[-1]['live']==ss[-1]['baseline']==s['input_build']['memory']['live_start']
                    assert ss[-3]['live']==s['execute_observe']['memory']['live_end']
                    assert ss[-2]['live']==s['consumer_drop']['memory']['live_end']
                    assert ss[-1]['live']==s['engine_drop']['memory']['live_end']
                    keep=count if c[5]=='all' else min(count,int(c[5]))
                    assert ss[-3]['retained']==keep and ss[-2]['retained']==0
            if v['meter']:assert v['source_build']['memory']['live_start']==v['prepared_drop']['memory']['live_end']
            else:assert not v['snapshots']
            data[kind,index]=v;hashes[path.name]=sha(path)
    results=json.loads((OUT/'summary.json').read_text());assert results['exploratory'];assert len(results['results'])==144
    def phases(v):return [v['preparation'],v['prepared_drop']]+[s[k] for s in v['samples'] for k in ['setup','execute_observe','consumer_drop','engine_drop']]
    groups={}
    for i,row in enumerate(results['results']):
        assert row['config']==configs[i]
        for j,m in enumerate(modes):
            a=data['allocation',11*i+j];v=data['ordinary',11*i+j]
            assert strip(a)==strip(data['allocation',1584+11*i+j])
            r=row['modes'][m];ns=sum(p['ns'] for p in phases(v))
            assert r['primary_ms']==ns/1e6
            assert r['inclusive_ms']==(ns+v['source_build']['ns']+sum(s['input_build']['ns'] for s in v['samples']))/1e6
            assert r['requested_bytes']==sum(p['memory']['requested_bytes'] for p in phases(a))
            all_ps=phases(a)+[a['source_build']]+[s['input_build'] for s in a['samples']]
            assert r['peak_growth']==max(p['memory']['peak_live'] for p in all_ps)-a['source_build']['memory']['live_start']
            assert r['owner_snapshots']==a['snapshots']
            for s in a['snapshots']:
                if s['label']=='consumer-released':
                    groups.setdefault(tuple(configs[i][:5])+ (m,s['query']),[]).append(s['live']-s['baseline'])
    assert len(groups)==1056 and all(len(v)==3 and len(set(v))==1 for v in groups.values())
    (OUT/'audit.json').write_text(json.dumps({'processes':4818,'exact_allocation_replays':1584,'consumer_independent_owner_groups':1056,'source_files':len(f['sources']),'receipt_hashes':hashes},indent=2)+'\n')
    print('4818 receipts;1584 exact allocation replays;1056 consumer-independent owner groups verified')
if __name__=='__main__':main()
