"""Reject omitted source-read dependencies with independent complete answers."""
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-matcher-settlement'
def main():
    p=ROOT/'research/chr-relational/src/store.rs';original=p.read_text();receipts=[]
    mutations=[('guard','counts[v] > 1 || guard_vars.contains(v)','counts[v] > 1','matcher_settlement_preserves_the_priority_witness'),('repeated','counts[v] > 1 || guard_vars.contains(v)','guard_vars.contains(v)','matcher_settlement_covers_new_posts_constructors_and_repeated_variables'),('constructor','Term::App(..) => true,','Term::App(..) => false,','matcher_settlement_covers_new_posts_constructors_and_repeated_variables')]
    try:
        for label,old,new,test in mutations:
            assert original.count(old)==1
            p.write_text(original.replace(old,new))
            r=subprocess.run(['cargo','test','-p','chr-relational','--lib',test],cwd=ROOT,capture_output=True,text=True)
            (OUT/f'mutation-{label}.log').write_text((r.stdout+r.stderr).rstrip()+'\n')
            assert r.returncode==101 and 'test result: FAILED' in r.stdout and 'error[E' not in r.stderr,(label,r.stdout,r.stderr)
            receipts.append(dict(omitted=label,test=test,exit_code=r.returncode))
    finally:p.write_text(original)
    h=hashlib.sha256(p.read_bytes()).hexdigest()
    assert h==hashlib.sha256(original.encode()).hexdigest()
    (OUT/'mutation-audit.json').write_text(json.dumps(dict(source_restored_sha256=h,rejected=receipts),indent=2)+'\n')
if __name__=='__main__':main()
