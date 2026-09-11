"""Negative controls: each cache must be cleared at the service boundary."""
import json
import subprocess
from pathlib import Path
from continuing_lifecycle_entry import ROOT, sha

OUT=ROOT/'docs/experiments/results/s08-completed-traversal'

def main():
    p=ROOT/'research/chr-direct-choice/src/demand.rs';original=p.read_text();before=sha(p)
    receipts=[]
    try:
        for clear,test in [('self.finite_done.clear();','completed_cache_does_not_hide_a_later_constructor_cycle'),('self.completed_force.clear();','completed_cache_does_not_cross_choice_contexts')]:
            assert original.count(clear)==1
            p.write_text(original.replace(clear,'/* negative control: stale entries survive */'))
            cmd=['cargo','test','-p','chr-direct-choice','--features','completed-traversal,work-diagnostics',test,'--lib']
            r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
            (OUT/f'mutation-{test}.log').write_text((r.stdout+r.stderr).rstrip()+'\n')
            assert r.returncode==101 and 'test result: FAILED' in r.stdout and 'error[E' not in r.stderr
            receipts.append(dict(mutation=clear,test=test,exit_code=r.returncode))
    finally:
        p.write_text(original)
    assert sha(p)==before
    (OUT/'mutation-audit.json').write_text(json.dumps(dict(source_restored_sha256=before,rejected=receipts),indent=2)+'\n')

if __name__=='__main__':main()
