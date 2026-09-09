"""Registered allocation-only stream ownership gate; exact replay, no timings."""
import hashlib
import itertools
import json
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import derivation_sizing as limits
OUT = ROOT/'docs/experiments/results/s08-stream-ownership-gate'
BIN = Path('/tmp/chr-stream-ownership-6e3289cf6')
MODES = ['direct','compact-live','scan','sealed','dependencies','templates','lowered']
CONFIGS = [(m,f,n,r,k,0) for f,n,r,k,m in itertools.product(
    ['repeated','distinct','aliases'],[16,64],[0,1],['0','4','all'],MODES)]
CANCEL = [(m,'aliases',64,1,k,1) for k,m in itertools.product(['0','4','all'],MODES)]


def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    OUT.mkdir(exist_ok=False)
    shutil.copyfile(ROOT/'target/release/examples/stream_ownership',BIN)
    BIN.chmod(0o755)
    files=['Cargo.toml','Cargo.lock','docs/experiments/registrations/S08-stream-ownership-gate.md']
    for d in ['crates/chr-syntax','research/chr-reuse','research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-direct-choice','research/chr-direct-conditional']:
        files += [str(p.relative_to(ROOT)) for p in (ROOT/d).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.py']]
    freeze={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
        'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':limits.CPU,
        'sources':{},'binary':{str(BIN):sha(BIN)},'configs':CONFIGS,'cancel':CANCEL}
    for name in sorted(set(files)):
        dest=OUT/'source'/name;dest.parent.mkdir(parents=True,exist_ok=True)
        shutil.copyfile(ROOT/name,dest);freeze['sources'][name]=sha(dest)
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    check=subprocess.run([str(BIN),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=limits.limits)
    (OUT/'meter-check.log').write_text(check.stdout+check.stderr);assert check.returncode==0
    values={}
    for kind,configs in [('complete',CONFIGS),('cancel',CANCEL)]:
        for rep in range(2):
            for i,c in enumerate(configs):
                cmd=[str(BIN)]+list(map(str,c));dest=OUT/f'{kind}-{rep}-{i:03}.json'
                try:
                    p=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits.limits)
                except subprocess.TimeoutExpired as e:
                    dest.write_text(json.dumps({'command':cmd,'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)},indent=2)+'\n');raise
                dest.write_text(json.dumps({'command':cmd,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr},indent=2)+'\n')
                assert p.returncode==0,dest
                v=json.loads(p.stdout.splitlines()[-1]);assert v['meter'] and not v['counters']
                if rep: assert v==values[kind,i],dest
                else: values[kind,i]=v
            print(kind,'repetition',rep+1,'complete',flush=True)
    summary=[]
    for c in CONFIGS:
        v=values['complete',CONFIGS.index(c)]
        summary.append({'config':c,'snapshots':[{'query':s['query'],'label':s['label'],'answers':s['answers'],'retained':s['retained'],'live_growth':s['memory']['live_end']-s['baseline'],'requested_bytes':s['memory']['requested_bytes']} for s in v['snapshots']]})
    (OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    for name,h in freeze['sources'].items():assert sha(ROOT/name)==h
    print('546 successful processes; 273 exact allocation replays',flush=True)


if __name__=='__main__':main()
