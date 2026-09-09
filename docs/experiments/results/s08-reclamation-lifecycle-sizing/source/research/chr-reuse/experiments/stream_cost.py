"""Registered exploratory lifecycle sizing; no confirmatory timing classifications."""
import hashlib,itertools,json,random,shutil,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import derivation_sizing as runner
OUT=ROOT/'docs/experiments/results/s08-reclamation-lifecycle-sizing'
BIN=Path('/tmp/chr-stream-cost-a31f14574')
MODES=['direct','compact-live','scan','sealed','dependencies','dependencies-reclaim','dependencies-periodic','templates','templates-reclaim','templates-periodic','lowered']
CONFIGS=[(f,n,w,p,r,k,0) for f,n,w,p,r,k in itertools.product(['repeated','distinct','aliases'],[16,64],[0,4],[0,8],[0,1],['0','4','all'])]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def untimed(v):
    if isinstance(v,dict):return {k:untimed(x) for k,x in v.items() if k not in ['ns','first_answer_ns']}
    if isinstance(v,list):return [untimed(x) for x in v]
    return v
def phases(v):
    return [v['preparation'],v['prepared_drop']]+[s[k] for s in v['samples'] for k in ['setup','execute_observe','consumer_drop','engine_drop']]
def primary(v):return sum(p['ns'] for p in phases(v))
def run(kind,index,mode,c):
    binary=BIN/('meter' if kind in ['allocation','cancel-meter'] else 'ordinary')
    cmd=[str(binary),mode]+list(map(str,c));dest=OUT/f'{kind}-{index:04}.json';assert not dest.exists()
    try:p=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=runner.limits)
    except subprocess.TimeoutExpired as e:
        dest.write_text(json.dumps({'command':cmd,'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)},indent=2)+'\n');raise
    dest.write_text(json.dumps({'command':cmd,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr},indent=2)+'\n');assert p.returncode==0,dest
    v=json.loads(p.stdout.splitlines()[-1]);assert not v['counters'] and v['meter']==(binary.name=='meter')
    assert (v['family'],v['depth'],v['work'],v['payload'],int(v['resource']),v['keep'],int(v['cancel']))==tuple(c)
    assert v['mode']==mode and len(v['samples'])==2
    for q,s in enumerate(v['samples']):
        assert s['depth']==c[1]+q
        assert s['complete']==(not c[6] or q==1)
        assert s['answers']==(4 if c[6] and q==0 else c[1]+q+1)
    return v
def main():
    assert len(CONFIGS)==144
    OUT.mkdir(exist_ok=False)
    files=['Cargo.toml','Cargo.lock','docs/experiments/registrations/S08-reclamation-lifecycle-sizing.md']
    for d in ['crates/chr-syntax','research/chr-reuse','research/chr-persistent','research/chr-compiled','research/chr-observe','research/chr-direct-choice','research/chr-direct-conditional']:
        files += [str(p.relative_to(ROOT)) for p in (ROOT/d).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.py']]
    f={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':runner.CPU,'configs':CONFIGS,'modes':MODES,'sources':{},'binaries':{}}
    for name in sorted(set(files)):
        dst=OUT/'source'/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/name,dst);f['sources'][name]=sha(dst)
    for name in ['ordinary','meter']:
        f['binaries'][str(BIN/name)]=sha(BIN/name);shutil.copyfile(BIN/(name+'.log'),OUT/(name+'-build.log'))
    (OUT/'freeze.json').write_text(json.dumps(f,indent=2)+'\n')
    order=list(range(1584));random.Random(7911).shuffle(order)
    (OUT/'ordinary-order.json').write_text(json.dumps(order)+'\n')
    p=subprocess.run([str(BIN/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=runner.limits)
    (OUT/'meter-check.log').write_text(p.stdout+p.stderr);assert p.returncode==0
    allocations={}
    for rep in range(2):
        for i,c in enumerate(CONFIGS):
            for j,m in enumerate(MODES):
                index=11*i+j;v=run('allocation',rep*1584+index,m,c)
                if rep:assert untimed(v)==untimed(allocations[index])
                else:allocations[index]=v
        print('allocation',rep+1,'complete',flush=True)
    ordinary={}
    for count,index in enumerate(order,1):
        i,j=divmod(index,11);ordinary[index]=run('ordinary',index,MODES[j],CONFIGS[i])
        if count%396==0:print(count,'ordinary processes complete',flush=True)
    for i,k in enumerate(['0','4','all']):
        for j,m in enumerate(MODES):
            for kind in ['cancel','cancel-meter']:run(kind,i*11+j,m,('aliases',64,4,8,1,k,1))
    results=[]
    for i,c in enumerate(CONFIGS):
        row={'config':c,'modes':{}}
        for j,m in enumerate(MODES):
            a=allocations[i*11+j];v=ordinary[i*11+j]
            all_ps=phases(a)+[a['source_build']]+[s['input_build'] for s in a['samples']]
            row['modes'][m]={'primary_ms':primary(v)/1e6,
                'inclusive_ms':(primary(v)+v['source_build']['ns']+sum(s['input_build']['ns'] for s in v['samples']))/1e6,
                'requested_bytes':sum(p['memory']['requested_bytes'] for p in phases(a)),
                'peak_growth':max(p['memory']['peak_live'] for p in all_ps)-a['source_build']['memory']['live_start'],
                'phase_ms':{'preparation':v['preparation']['ns']/1e6,'prepared_drop':v['prepared_drop']['ns']/1e6,
                    **{k:sum(s[k]['ns'] for s in v['samples'])/1e6 for k in ['setup','execute_observe','consumer_drop','engine_drop']}},
                'first_answer_ms':[s['first_answer_ns']/1e6 for s in v['samples']],
                'owner_snapshots':a['snapshots']}
        results.append(row)
    (OUT/'summary.json').write_text(json.dumps({'exploratory':True,'results':results},indent=2)+'\n')
    for name,h in f['sources'].items():assert sha(ROOT/name)==h
    print('4818 processes complete;1584 exact allocation replays',flush=True)
if __name__=='__main__':main()
