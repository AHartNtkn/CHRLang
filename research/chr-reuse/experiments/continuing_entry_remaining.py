"""Complete independent entry jobs after the recorded conditional service cutoff."""
import gzip,hashlib,json
from continuing_lifecycle_entry import BASE,ROOT,invoke
from pathlib import Path
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    parent=json.loads((BASE/'freeze.json').read_text());done=[json.loads(s) for s in gzip.open(BASE/'qualification.jsonl.gz','rt')]
    assert len(done)==91 and done[-1]['raw']['exit_code']==101
    assert not (BASE/'remaining.jsonl.gz').exists()
    for k,b in parent['binaries'].items():assert sha(Path(b['path']))==b['sha256']
    (BASE/'remaining-freeze.json').write_text(json.dumps(dict(parent_sha256=sha(BASE/'freeze.json'),registration_sha256=sha(ROOT/'docs/experiments/registrations/S08-continuing-resource-attribution.md'),runner_sha256=sha(Path(__file__)),jobs=parent['jobs'][91:]),indent=2))
    with gzip.open(BASE/'remaining.jsonl.gz','wt') as out:
        for j in parent['jobs'][91:]:
            args=[j['mode'],j['resource'],j['demand'],'all',4,j['packing'],True]
            r=invoke([parent['binaries'][j['kind']]['path']]+[str(x).lower() for x in args],120)
            out.write(json.dumps(dict(job=j,raw=r))+'\n');out.flush();print(j,r['exit_code'],flush=True)
if __name__=='__main__':main()
