from pathlib import Path
import importlib.util,json,hashlib,sys
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2];RAW=ROOT/'docs/experiments/results/s03-descriptor-source'
def load(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m
if __name__=='__main__':
 load('cases',ROOT/'research/chr-hvm/source_identity/cases.py')
 gate=load('gate',ROOT/'research/chr-hvm/source_identity/gate.py');compiler=load('compiler',ROOT/'research/chr-hvm/source_identity/compiler.py')
 paths=[Path(__file__),ROOT/'research/chr-hvm/source_identity/compiler.py',ROOT/'research/chr-hvm/source_identity/gate.py',gate.BINARY,RAW/'cases.json',ROOT/'docs/experiments/registrations/S03-descriptor-source-controls.md']
 p=RAW/'native-manifest.json';assert not p.exists();p.write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2)+'\n')
 (RAW/'programs').mkdir();rows=[]
 for source in json.loads((RAW/'cases.json').read_text()):
  program,preds,atoms=compiler.compile_source({k:source[k] for k in ['rules','query','outputs']});p=RAW/'programs'/(source['name']+'.hvm');p.write_text(program)
  r=gate.native.invoke(gate.BINARY,p,8,65536);row=dict(case=source['name'],result=r);rows.append(row)
  (RAW/'native-controls.json').write_text(json.dumps(rows,indent=2)+'\n')
  assert r['code']==0 and len(r['stdout'].splitlines())==1,row
  answer=gate.parse(r['stdout'],preds,atoms);assert gate.normalize(answer)==gate.normalize(source['expected']),row
  assert json.loads(r['stderr'].splitlines()[-1])['pending']==0
  row['answer']=answer
 (RAW/'native-controls.json').write_text(json.dumps(rows,indent=2)+'\n');print('79 raw native source observations validated')
