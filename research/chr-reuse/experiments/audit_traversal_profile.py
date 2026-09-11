"""Verify complete child observations and archived diagnostic profiles."""
import gzip
import hashlib
import json
import re
import sys
from pathlib import Path
sys.dont_write_bytecode=True
from audit_continuing import lifecycle
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s08-traversal-profile'


def main():
    receipts=json.loads((OUT/'receipts.json').read_text());assert len(receipts)==4
    result=[]
    for r in receipts:
        stem=r['mode']+'-'+r['resource']
        with gzip.open(OUT/(stem+'.perf.data.gz'),'rb') as f:assert hashlib.sha256(f.read()).hexdigest()==r['perf_sha256']
        with gzip.open(OUT/(stem+'-outputs.jsonl.gz'),'rt') as f:rows=[json.loads(s) for s in f]
        assert len(rows)==r['processes'] and r['exit_code']==0
        for d in rows:lifecycle(dict(exit_code=0,stdout=json.dumps(d)),128,'0')
        report=(OUT/(stem+'-self.txt')).read_text();assert '# Total Lost Samples: 0' in report
        samples=int(re.search(r'\((\d+) samples\)',(OUT/(stem+'-perf.log')).read_text())[1])
        assert samples>=500
        values={}
        for name,pattern in [('finite_visit','finite_result5visit'),('force','Run5force'),('tick','Run4tick')]:
            lines=[s for s in report.splitlines() if pattern in s]
            assert len(lines)==1
            values[name]=float(re.search(r'([\d.]+)%',lines[0])[1])
        result.append(dict(mode=r['mode'],resource=r['resource'],samples=samples,self_cycle_percent=values))
    (OUT/'analysis.json').write_text(json.dumps(dict(child_processes=240,profiles=result),indent=2)+'\n')
    print(json.dumps(result,indent=2))


if __name__=='__main__':main()
