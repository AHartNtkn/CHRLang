"""Bounded diagnostic profiles on the frozen qualified lifecycle binary."""
import gzip
import hashlib
import json
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s08-traversal-profile'


def main():
    binary=ROOT/'target/s08-traversal-lifecycle-binaries/True-time-continuing'
    frozen=json.loads((ROOT/'docs/experiments/results/s08-traversal-lifecycle/freeze.json').read_text())
    expected=next(e['sources']['continuing']['sha256'] for e in frozen['binaries'] if e['enabled']=='True' and e['kind']=='time')
    assert hashlib.sha256(binary.read_bytes()).hexdigest()==expected
    receipts=[]
    for mode,reps in [('dependencies',20),('templates',100)]:
        for resource in ['false','true']:
            stem=mode+'-'+resource;data=Path('/tmp')/('chr-profile-'+stem+'.data')
            script='for ((i=0;i<$1;i++)); do "$2" "$3" "$4" 128 0 4 false false || exit; done'
            command=['taskset','-c','0','perf','record','-e','cycles:u','-F','999','--call-graph','dwarf','-o',str(data),'--','bash','-c',script,'profile',str(reps),str(binary),mode,resource]
            r=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=60)
            (OUT/(stem+'-perf.log')).write_text(r.stderr.rstrip()+'\n')
            with gzip.open(OUT/(stem+'-outputs.jsonl.gz'),'wt') as out:out.write(r.stdout)
            assert r.returncode==0,r.stderr
            rows=[json.loads(s) for s in r.stdout.splitlines()];assert len(rows)==reps and all(d['validated'] for d in rows)
            for kind,extra in [('self',['--no-children','--call-graph','none']),('inclusive',['--children','--call-graph','none'])]:
                report=subprocess.run(['perf','report','-i',str(data),'--stdio','--sort','symbol','--percent-limit','1',*extra],capture_output=True,text=True,check=True)
                (OUT/(stem+'-'+kind+'.txt')).write_text(report.stdout.rstrip()+'\n')
            with data.open('rb') as src,gzip.open(OUT/(stem+'.perf.data.gz'),'wb') as dst:dst.write(src.read())
            receipts.append(dict(mode=mode,resource=resource,processes=reps,command=command,exit_code=r.returncode,binary_sha256=expected,perf_sha256=hashlib.sha256(data.read_bytes()).hexdigest()))
            (OUT/'receipts.json').write_text(json.dumps(receipts,indent=2)+'\n')
            print(stem,'profile validated',flush=True)


if __name__=='__main__':main()
