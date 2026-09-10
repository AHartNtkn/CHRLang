"""Pinned native retained-graph service experiment; independent leaf expectations."""
import hashlib,json,os,resource,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s03-native-service'
CHECKOUT=Path('/tmp/chr-hvm4-research-20260906')
PIN='6defdfc7dae2a3cca5dd6e74ed0612385b5646a8'
CASES=[('atom','#A',['#A{}'],False),('duplicate','&(1){#A,#A}',['#A{}','#A{}'],False),('nested','&(1){&(2){#A,#B},&(3){#C,#D}}',['#A{}','#B{}','#C{}','#D{}'],False),('loop-first','&(1){@spin(#Loop),#A}',['#A{}'],True),('loop-last','&(1){#A,@spin(#Loop)}',['#A{}'],True),('loops','&(1){@spin(#Loop),@spin(#Loop)}',[],True),('siblings','&(1){&(2){@spin(#Loop),#A},&(3){#B,#C}}',['#A{}','#B{}','#C{}'],True),('reference-first','&(1){@loop,#A}',['#A{}'],True),('reference-last','&(1){#A,@loop}',['#A{}'],True),('structured','#Pair{#A,#B}',[],False)]
def limits():
 resource.setrlimit(resource.RLIMIT_CPU,(2,2));resource.setrlimit(resource.RLIMIT_AS,(96<<30,96<<30))
def run(binary,path,fuel=1,calls=64,counters_off=False):
 try:
  p=subprocess.run([str(binary),str(path),'-C'],capture_output=True,text=True,timeout=3,preexec_fn=limits,env=dict(os.environ,CHR_FUEL=str(fuel),CHR_CALLS=str(calls),**({"CHR_COUNTERS_OFF":"1"} if counters_off else {})))
  return dict(code=p.returncode,stdout=p.stdout,stderr=p.stderr)
 except subprocess.TimeoutExpired:return dict(timeout=True)
def main():
 out=OUT;out.mkdir(parents=True,exist_ok=True)
 build=ROOT/'target/s03-native-service';build.mkdir(parents=True,exist_ok=True)
 source=CHECKOUT/'src/hvm.c';assert subprocess.check_output(['git','-C',str(CHECKOUT),'rev-parse','HEAD'],text=True).strip()==PIN
 original=source.read_text();assert hashlib.sha256(source.read_bytes()).hexdigest()=='3d2724d0716b5d6b1c07a3b0b3c5cb89f848487a43d364ea689658f9aeb75fe7'
 baseline=build/'baseline'
 subprocess.run(['clang','-O2',str(source),'-o',str(baseline)],check=True,timeout=60)
 programs={}
 for name,expr,_,_ in CASES:
  p=out/(name+'.hvm');p.write_text('@spin = λx.@spin(x)\n@loop = @loop\n@main = '+expr+'\n');programs[name]=p
 if sys.argv[-1]=='red':
  r=run(baseline,programs['loop-first']);(out/'red.json').write_text(json.dumps(r,indent=2)+'\n')
  assert '"pending"' in r.get('stdout',''),'Unmodified collapse does not return native service state on looping-first source'
  return
 fragment=Path(__file__).with_name('scheduler.c').read_text()
 patched=original.replace('fn void eval_collapse(Term root, int limit, int stats, int silent);','fn void eval_collapse(Term root, int limit, int stats, int silent);\nfn void chr_service(Term root);',1)
 patched=patched.replace('eval_collapse(main_ref, run.collapse_limit, run.stats, run.silent);','chr_service(main_ref);',1)
 assert patched.count('  enter: {') == 1
 patched=patched.replace('  enter: {', '  enter: {\n    if (CHR_ACTIVE && CHR_VISITS >= CHR_QUOTA) { CHR_YIELDED=1; return wnf_rebuild(next, stack, s_pos, base); }',1)
 hook='    while (s_pos > base) {'
 assert patched.count(hook)==1
 patched=patched.replace(hook,hook+'\n      if (CHR_ACTIVE && CHR_VISITS >= CHR_QUOTA) { CHR_YIELDED=1; return wnf_rebuild(whnf, stack, s_pos, base); }',1)
 # Charge rewrites independently of optional diagnostic counters, and references
 # that can otherwise loop without producing an interaction.
 marker='      case REF: {'
 a=patched.index('__attribute__((hot)) fn Term wnf(Term term)')
 b=patched.index('fn Term wnf_at',a)
 reducer=patched[a:b];assert reducer.count(marker)==1
 reducer=reducer.replace(marker,marker+'\n        if (CHR_ACTIVE) CHR_VISITS++;',1)
 patched=patched[:a]+reducer+patched[b:]
 marker='    if (ITRS_ENABLED != 0) {'
 assert patched.count(marker)==1
 patched=patched.replace(marker,'    if (CHR_ACTIVE) CHR_VISITS++; '+chr(92)+'\n'+marker,1)
 patched='static unsigned long long CHR_QUOTA, CHR_VISITS;\nstatic int CHR_ACTIVE, CHR_YIELDED;\n'+patched+'\n'+fragment
 generated=build/'service-transitions.c';generated.write_text(patched);binary=build/'service-transitions'
 subprocess.run(['clang','-O2',str(generated),'-o',str(binary)],check=True,timeout=60)
 rows=[]
 for replay in range(2):
  for fuel in [1,2,8]:
   for name,_,expected,ongoing in CASES:
    r=run(binary,programs[name],fuel);row=dict(replay=replay,fuel=fuel,case=name,result=r);rows.append(row)
    (out/'runs.json').write_text(json.dumps(rows,indent=2)+'\n')
    assert r.get('code')==0,row
    events=[json.loads(line) for line in r['stderr'].splitlines()];row['events']=events
    answers=[l for l in r['stdout'].splitlines() if l and not l.startswith('- ')]
    assert sorted(answers)==sorted(expected),row
    end=events[-1];assert bool(end['pending'])==ongoing,row
    assert end['unsupported']==int(name=='structured'),row
    assert all(e['visits']<=fuel and e['stack']==1 for e in events[:-1]),row
 for name in programs:
  r=run(binary,programs[name],1,0);ev=json.loads(r['stderr']);assert ev['pending']==1 and ev['calls']==0 and ev['unsupported']==0,r
  rows.append(dict(case=name,cancel_zero=r))
 for i in range(30):assert rows[i]['result']==rows[i+30]['result'],rows[i]
 counter_off=[]
 for row in rows[:30]:
  r=run(binary,programs[row['case']],row['fuel'],counters_off=True)
  assert r.get('code')==0,r
  assert r['stdout']==row['result']['stdout'],r
  events=[json.loads(line) for line in r['stderr'].splitlines()]
  original=row['events']
  assert [{k:v for k,v in e.items() if k!='delta'} for e in events]==[{k:v for k,v in e.items() if k!='delta'} for e in original],r
  assert all(e['delta']==0 for e in events[:-1]),r
  counter_off.append(dict(case=row['case'],fuel=row['fuel'],result=r))
 (out/'counter-off.json').write_text(json.dumps(counter_off,indent=2)+'\n')
 controls=[]
 for name,_,expected,ongoing in CASES:
  if ongoing or name=='structured':continue
  r=run(baseline,programs[name]);assert r.get('code')==0,r
  # Stats annotations follow a completed nullary constructor.
  values=[l.split(' ')[0] for l in r['stdout'].splitlines() if l and not l.startswith('- ')]
  assert sorted(values)==sorted(expected),(name,r)
  controls.append(dict(case=name,result=r))
 (out/'runs.json').write_text(json.dumps(rows,indent=2)+'\n');(out/'controls.json').write_text(json.dumps(controls,indent=2)+'\n')
 hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path(__file__),Path(__file__).with_name('scheduler.c'),binary,baseline,*programs.values()]}
 (out/'validation.json').write_text(json.dumps(dict(pin=PIN,upstream_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),compiler=subprocess.check_output(['clang','--version'],text=True),hashes=hashes,cells=30,replays=2,zero_cancellations=10,controls=len(controls),counter_off=30),indent=2)+'\n')
 print('30 native service cases replay exactly; ten zero-call cancellations and three finite controls pass.')
if __name__=='__main__':main()
