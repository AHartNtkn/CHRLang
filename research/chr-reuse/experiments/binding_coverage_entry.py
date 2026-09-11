"""Qualify aggregate binding coverage against frozen continuing-source controls."""
import gzip,hashlib,itertools,json,subprocess,zipfile
from pathlib import Path
from continuing_lifecycle_entry import ROOT,BASE,invoke

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    assert not (BASE/'coverage-freeze.json').exists()
    bins={k:ROOT/f'target/s08-coverage-{k}/release/examples/{"continuing_resource_stages" if k=="diagnostic" else "continuing_lifecycle"}' for k in ['diagnostic','time','meter']}
    jobs=[dict(kind='diagnostic',resource=r,demand=n,rep=rep) for r,n,rep in itertools.product([False,True],[128,512],range(2))]
    jobs += [dict(kind=k,resource=r,demand=n,packing=p) for n,packing in [(32,[False,True]),(512,[False])] for r,p,k in itertools.product([False,True],packing,['time','meter'])]
    assert len(jobs)==20
    paths=[]
    for folder in ['research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths.extend(p for p in (ROOT/folder).rglob('*.rs') if 'target' not in p.parts);paths.append(ROOT/folder/'Cargo.toml')
    paths.extend([ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S08-binding-coverage.md'])
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(BASE/'coverage-sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (BASE/'coverage-freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'coverage-sources.zip'),jobs=jobs,binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()}),indent=2))
    with gzip.open(BASE/'coverage.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            args=[j['resource'],j['demand']] if j['kind']=='diagnostic' else ['conditional',j['resource'],j['demand'],'all',4,j['packing'],True]
            raw=invoke([str(bins[j['kind']])]+[str(x).lower() for x in args],120 if j['demand']==512 or j['kind']=='diagnostic' else 60)
            out.write(json.dumps(dict(job=j,raw=raw))+'\n');out.flush();print(i,j,raw['exit_code'],flush=True)
            assert raw['exit_code']==0,(j,raw['stderr'])
if __name__=='__main__':main()
