"""Exact phase attribution for generated source identity checking."""
from pathlib import Path
import json,sys
ROOT=Path(__file__).resolve().parents[3]
OLD=ROOT/'docs/experiments/results/s05-call-generated-entry'
NEW=ROOT/'docs/experiments/results/s05-call-generated-source-check'
PREVIOUS=ROOT/'docs/experiments/results/s05-call-compiled-cost'
def phases(path):
    r=json.loads(path.read_text());assert r['returncode']==0
    out={}
    for line in r['stdout'].splitlines()[1:]:
        row=json.loads(line);key=row['phase'];out[key]=out.get(key,0)+row['memory']['requested_bytes']
    return out

def analyze():
    before=json.loads((OLD/'analysis.json').read_text());after=json.loads((NEW/'analysis.json').read_text())
    prior=json.loads((PREVIOUS/'analysis.json').read_text())['comparisons']
    controls={(tuple(x['scenario']),x['control']):x['control_memory']for x in prior}
    cells=json.loads((PREVIOUS/'freeze.json').read_text())['cells']
    out=[]
    for i,(a,b)in enumerate(zip(before,after,strict=True)):
        assert a['cell']==b['cell'];c=a['cell'];control='indexed'if c[0]=='generated'else'sealed'
        pa=phases(OLD/f'meter-0-{i:03}.json');pb=phases(NEW/f'meter-0-{i:03}.json')
        assert pa.keys()==pb.keys()
        assert all(pa[k]==pb[k]for k in pa if k!='prepare')
        saved=pa['prepare']-pb['prepare'];assert saved>0
        assert a['memory']['requested']-b['memory']['requested']==saved
        assert a['memory']['consumer']==b['memory']['consumer']
        oi=cells.index([control,*c[1:]])
        pc=phases(PREVIOUS/f'meter-00-{oi:03}.json')
        out.append(dict(cell=c,control=control,before=a['memory'],after=b['memory'],generic=controls[tuple(c[1:]),control],preparation_bytes_saved=saved,before_phases=pa,after_phases=pb,generic_phases=pc))
    return out
if __name__=='__main__':
    path=NEW/'diagnosis.json'
    if sys.argv[1]=='run':
        assert not path.exists();path.write_text(json.dumps(analyze(),indent=2)+'\n')
    else:
        assert analyze()==json.loads(path.read_text());print('288 phase attributions verified; allocation reductions confined to preparation.')
