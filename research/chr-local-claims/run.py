"""Bounded exhaustive protocol check with independent atomic outcomes."""
from pathlib import Path
import hashlib,json,resource,sys
sys.dont_write_bytecode=True
from model import CASES,explore
from oracle import outcomes
ROOT=Path(__file__).resolve().parents[2]
RAW=ROOT/'docs/experiments/results/s03-local-claims'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
    paths=list(Path(__file__).parent.glob('*.py'))+[ROOT/'docs/experiments/registrations/S03-local-claims.md']
    manifest=RAW/'manifest.json';assert not manifest.exists()
    manifest.write_text(json.dumps(dict(cases={k:[n,b,[a.__dict__ for a in apps]] for k,(n,b,apps) in CASES.items()},sha256={str(p.relative_to(ROOT)):digest(p) for p in paths}),indent=2)+'\n')
    rows=[];total=0
    try:
        for name,case in CASES.items():
            for cancel in [None,*range(len(case[2]))]:
                serial=outcomes(case,cancel,True)
                for ordered in [False,True]:
                    got,states,edges,partial,traces=explore(case,cancel,ordered);expected=outcomes(case,cancel,ordered)
                    assert got==expected,(name,cancel,ordered,'oracle mismatch',got-expected,expected-got)
                    total+=states;assert total<=2000000,'total state cutoff'
                    differences=got-serial
                    rows.append(dict(case=name,cancel=cancel,priority_admission=ordered,states=states,edges=edges,partial_cancel_edges=partial,terminal_outcomes=len(got),outside_source_priority=len(differences),witnesses=[dict(observation=o,trace=traces[o]) for o in sorted(differences)]))
        assert any(r['partial_cancel_edges'] for r in rows)
        # Independent known observations for the simplest fragments.
        assert {x[2] for x in outcomes(CASES['disjoint'],None,False)}=={(0,1)}
        assert {x[2] for x in outcomes(CASES['contended'],None,False)}=={(0,),(1,)}
        assert {x[2] for x in outcomes(CASES['contended'],None,True)}=={(0,)}
        result=dict(status='passed',cells=len(rows),states=total,rows=rows)
    except Exception as e:
        result=dict(status='failed',error=repr(e),completed=rows)
        (RAW/'results.json').write_text(json.dumps(result,indent=2)+'\n');raise
    (RAW/'results.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:v for k,v in result.items() if k!='rows'}))
if __name__=='__main__':run()
