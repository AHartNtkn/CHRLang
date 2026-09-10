"""Verify frozen bounded semantic receipts; this is not a timing analyzer."""
from pathlib import Path
import hashlib,json,re
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s06-projection-entry/qualified'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
if __name__=='__main__':
    manifest=json.loads((RAW/'manifest.json').read_text())
    for p,h in manifest['sha256'].items():assert digest(ROOT/p)==h,p
    rows=[];names={}
    for build,test,binary in manifest['binaries']:
        path=RAW/f'{build}-{test}.json';r=json.loads(path.read_text())
        assert r['command']==[str(ROOT/binary),'--test-threads=1','--nocapture']
        assert r['returncode']==0 and not r['stderr']
        count=10 if test=='projection' else 16
        assert f'test result: ok. {count} passed; 0 failed;' in r['stdout']
        tests=re.findall(r'^test (\S+) \.\.\.',r['stdout'],re.M);assert len(tests)==count
        if test in names:assert names[test]==tests
        else:names[test]=tests
        rows.append(dict(build=build,test=test,passed=count,receipt_sha256=digest(path)))
    assert {(r['build'],r['test']) for r in rows}=={(b,t) for b in ['default','counter-free'] for t in ['projection','finite_paths']}
    audit=dict(status='passed',bounded_processes=4,rows=rows,generated_problems_per_build=256,visible_selections_per_build=2048,order_semantics_comparisons_per_build=8192,scope='semantic and work-count gate; no comparative cost runs')
    (RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n');print('Verified four bounded binaries: 10 projection and 16 finite-path tests in each feature configuration.')
