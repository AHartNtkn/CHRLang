"""Recheck pilot positions, semantics and registered descriptive timing screen."""
import hashlib,json,random,statistics
from collections import Counter
from gate import ROOT,OUT,BUILD,rust,wire,fields

def main():
    receipt=json.loads((OUT/'validation.json').read_text())
    for p,h in receipt['hashes'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
    candidates=json.loads((OUT/'selection.json').read_text())
    rows=[json.loads(l) for l in (OUT/'pilot.jsonl').read_text().splitlines()]
    assert len(candidates)==27 and len(rows)==648
    warm=[(i,k,c,-1) for i in range(27) for k in ['primary','diagnostic'] for c in [0,1]]
    jobs=[(i,k,c,r) for i in range(27) for k in ['primary','diagnostic'] for c in [0,1] for r in range(5)]
    random.Random(20260911).shuffle(jobs)
    assert [(r['candidate'],r['kind'],r['cpu'],r['repetition']) for r in rows]==warm+jobs
    for r in rows:
        c=candidates[r['candidate']];s=c['source'];result=r['result'];assert result['code']==0
        fields(result,r['kind'],c['language'])
        if c['language']=='rust':
            rust.check(result,[s]);e=json.loads(result['stderr'].splitlines()[1])
        else:
            payload=wire.records(bytes.fromhex(result['stdout_hex']));assert set(payload)=={0}
            got=wire.decode(payload[0],c['predicates'],c['atoms'])
            assert sorted(map(wire.common.normalize,got))==sorted(map(wire.common.normalize,s['expected']))
            e=json.loads(result['stderr'].splitlines()[0]);assert not e['pending'] and not e['unsupported']
        assert r['service_ns']==e['service_ns']
    summary=[]
    for i,c in enumerate(candidates):
        for cpu in [0,1]:
            a=[r['service_ns'] for r in rows if r['candidate']==i and r['cpu']==cpu and r['kind']=='primary' and r['repetition']>=0]
            b=[r['service_ns'] for r in rows if r['candidate']==i and r['cpu']==cpu and r['kind']=='diagnostic' and r['repetition']>=0]
            assert len(a)==len(b)==5
            ratio=statistics.median(b)/statistics.median(a)
            status='diagnostic_higher' if min(b)>max(a) and ratio>=1.1 else 'primary_higher' if min(a)>max(b) and ratio<=1/1.1 else 'unresolved'
            summary.append(dict(candidate=i,cpu=cpu,language=c['language'],mode=c['mode'],role=c['role'],primary_range=[min(a),max(a)],diagnostic_range=[min(b),max(b)],primary_median=statistics.median(a),diagnostic_median=statistics.median(b),ratio=ratio,status=status))
    (OUT/'analysis.json').write_text(json.dumps(summary,indent=2)+'\n')
    print('648 pilot/warmup processes independently replayed;',dict(Counter(r['status'] for r in summary)))
if __name__=='__main__':main()
