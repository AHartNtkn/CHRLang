"""Recheck frozen semantic confirmations and the reported work formulas."""
import hashlib,json,re,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-partial-priority'
def sha(b):return hashlib.sha256(b).hexdigest()
def main():
    f=json.loads((OUT/'freeze.json').read_text())
    assert sha((OUT/'sources.zip').read_bytes())==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(z.read(p))==h
    observed=[]
    for rep in range(2):
        r=json.loads((OUT/f'confirmation-{rep}.json').read_text());assert r['exit_code']==0
        assert 'test result: ok. 7 passed; 0 failed' in r['stdout']
        priority=re.findall(r'PRIORITY,depth=(\d+),early=(true|false),clash=(true|false),tokens=(\d+),eager=(\d+),settled=(\d+),pending_at_claim=(\d+)',r['stdout'])
        controls=re.findall(r'depth=(\d+) deep=(true|false) fail=(true|false) interleaved=(\d+) settled=(\d+)',r['stdout'])
        assert len(priority)==24 and len(controls)==12
        for depth,early,clash,tokens,eager,settled,pending in priority:
            depth=int(depth);assert int(eager)==depth+4
            assert int(settled)==depth+(3 if clash=='true' else 4)
            assert int(pending)>0
        for depth,deep,fail,eager,settled in controls:
            depth=int(depth);assert int(settled)==depth+1
            assert int(eager)==(2 if deep=='false' and fail=='true' else depth+1)
        observed.append((priority,controls))
    assert observed[0]==observed[1]
    print('Frozen inputs, two seven-test confirmations, 24 priority cases and 12 established controls verified')
if __name__=='__main__':main()
