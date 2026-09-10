"""Same-process prepared sessions, including return to an earlier ruleset."""
import json,subprocess
from gate import ROOT,OUT,BINARY,bounds,output_records

def main():
 plans=json.loads((OUT/'plans.json').read_text());selected=[plans[0],plans[-1],plans[0]]
 command=[str(BINARY),'--sessions']+[str(ROOT/p['program']) for p in selected]
 p=subprocess.run(command,input=''.join(s['input'] for s in selected),text=True,capture_output=True,timeout=15,preexec_fn=bounds)
 (OUT/'multi-session.json').write_text(json.dumps(dict(command=command,groups=[s['group'] for s in selected],code=p.returncode,stdout=p.stdout,stderr=p.stderr),indent=2)+'\n');assert p.returncode==0,p.stderr[-2000:]
 original={r['group']:r for r in map(json.loads,(OUT/'runs.jsonl').read_text().splitlines())}
 sections=p.stdout.split('SESSION ');assert sections[0]=='' and len(sections)==4
 for i,(section,plan) in enumerate(zip(sections[1:],selected)):
  index,body=section.split('\n',1);assert int(index)==i and body==original[plan['group']]['stdout']
 assert p.stderr==''.join(original[s['group']]['stderr'] for s in selected)
 print('Three same-process sessions reproduce standalone outputs and ownership receipts exactly.')
if __name__=='__main__':main()
