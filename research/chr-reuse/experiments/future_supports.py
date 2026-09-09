"""Run and audit the registered non-mutating support inventory."""
import hashlib,json,shutil,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
from derivation_sizing import limits,CPU
OUT=ROOT/'docs/experiments/results/s08-future-support-inventory'
BIN=Path('/tmp/chr-future-supports-b2b3b7fdd')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    OUT.mkdir(exist_ok=False)
    shutil.copyfile(ROOT/'target/release/examples/stream_supports',BIN);BIN.chmod(0o755)
    files=['Cargo.toml','Cargo.lock','docs/experiments/registrations/S08-future-support-inventory.md']
    for d in ['crates/chr-syntax','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-persistent','research/chr-compiled','research/chr-observe']:
        files += [str(p.relative_to(ROOT)) for p in (ROOT/d).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.py']]
    freeze={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':CPU,'sources':{},'binary':{str(BIN):sha(BIN)}}
    for name in sorted(set(files)):
        dst=OUT/'source'/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/name,dst);freeze['sources'][name]=sha(dst)
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    saved=None
    for rep in range(2):
        try:p=subprocess.run([str(BIN)],capture_output=True,text=True,timeout=60,preexec_fn=limits)
        except subprocess.TimeoutExpired as e:
            (OUT/f'run-{rep}.json').write_text(json.dumps({'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)},indent=2)+'\n');raise
        (OUT/f'run-{rep}.json').write_text(json.dumps({'command':[str(BIN)],'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr},indent=2)+'\n');assert p.returncode==0
        lines=[json.loads(s) for s in p.stdout.splitlines()]
        if saved is None:saved=lines
        else:assert lines==saved
    groups=[]
    for line in saved:
        if 'family' in line:groups.append({'config':line,'snapshots':[]})
        else:groups[-1]['snapshots'].append(line)
    assert len(groups)==96
    for g in groups:
        c=g['config'];ss=g['snapshots'];count=c['depth']+int(not c['fail_tail'])
        assert ss[0]['label']=='setup' and ss[-1]['label']=='exhausted'
        assert ss[-1]['answers']==count and ss[-1]['tasks']==0
        assert [s['answers'] for s in ss if s['label']=='delivery']==[n for n in [1,4,16,64] if n<=count]
        for s in ss:
            for name in ['results','obligations','births','claims']:
                assert 0<=s['dead_'+name]<=s[name]
                if s['tasks']==0:assert s['dead_'+name]==s[name]
    assert len({tuple(sorted(g['config'].items())) for g in groups})==96
    (OUT/'summary.json').write_text(json.dumps(groups,indent=2)+'\n')
    for name,h in freeze['sources'].items():assert sha(ROOT/name)==h==sha(OUT/'source'/name)
    (OUT/'audit.json').write_text(json.dumps({'processes':2,'identical_query_trajectories':96,'snapshots':sum(len(g['snapshots']) for g in groups),'source_files':len(freeze['sources']),'binary_hash':sha(BIN)},indent=2)+'\n')
    print('Two identical diagnostic runs;96 complete independently checked query trajectories')
if __name__=='__main__':main()
