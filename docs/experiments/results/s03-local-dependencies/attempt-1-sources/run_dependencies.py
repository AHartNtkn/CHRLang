from pathlib import Path
import hashlib,json,resource,sys
sys.dont_write_bytecode=True
from model import CASES
from oracle import outcomes
from dependencies import explore_components
from publication import explore_publication
ROOT=Path(__file__).resolve().parents[2];RAW=ROOT/'docs/experiments/results/s03-local-dependencies'
if __name__=='__main__':
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
    paths=[Path(__file__).with_name(n) for n in ['run_dependencies.py','model.py','oracle.py','dependencies.py','publication.py']]+[ROOT/'docs/experiments/registrations/S03-local-dependencies.md']
    p=RAW/'manifest.json';assert not p.exists();p.write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2)+'\n')
    rows=[];pub=[];states=0
    try:
        for name,case in CASES.items():
            for cancel in [None,*range(len(case[2]))]:
                for binding_edges in [False,True]:
                    got,n,e,parallel,groups=explore_components(case,cancel,binding_edges);expected=outcomes(case,cancel,True);different=set(got)-expected;missing=expected-set(got)
                    row=dict(case=name,cancel=cancel,binding_edges=binding_edges,states=n,edges=e,parallel_admission_states=parallel,groups=groups,outcomes=len(got),missing=len(missing),differences=[dict(observation=o,trace=got[o]) for o in sorted(different)])
                    rows.append(row);states+=n
                    if binding_edges:assert not different and not missing,row
                    assert states<=2000000
        for cancel in [False,True]:
            for checked in [False,True]:
                row=dict(cancellation=cancel,validated_scan=checked,**explore_publication(checked,cancel));pub.append(row);states+=row['states']
                if checked:assert not row['mixed'] and row['published']==[(0,)*4,(1,)*4],row
                assert states<=2000000
        result=dict(status='passed',states=states,admission=rows,publication=pub)
    except Exception as e:
        (RAW/'failure.json').write_text(json.dumps(dict(error=repr(e),admission=rows,publication=pub),indent=2)+'\n');raise
    (RAW/'results.json').write_text(json.dumps(result,indent=2)+'\n');print('passed',states,'states',len(rows),'admission cells',len(pub),'publication cells')
