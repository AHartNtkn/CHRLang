"""Run the registered independent-artifact accounting gate (not a timing pilot)."""
from pathlib import Path
import hashlib, json, os, resource, subprocess, time
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-artifact-boundary'
TARGET=ROOT/'target/s01-artifact-boundary'
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def run(name,command,limit,env=None,expected=0):
    def bounded():resource.setrlimit(resource.RLIMIT_AS,(limit,limit))
    start=time.monotonic()
    try:
        p=subprocess.run([str(x) for x in command],cwd=ROOT,env=env,capture_output=True,text=True,timeout=60,preexec_fn=bounded)
    except subprocess.TimeoutExpired as error:
        record=dict(command=[str(x) for x in command],cutoff=True,stdout=str(error.stdout),stderr=str(error.stderr),timeout_seconds=60)
        (OUT/(name+'.json')).write_text(json.dumps(record,indent=2)+'\n')
        raise
    record=dict(command=[str(x) for x in command],exit_code=p.returncode,elapsed_seconds=time.monotonic()-start,address_space_bytes=limit,stdout=p.stdout,stderr=p.stderr)
    (OUT/(name+'.json')).write_text(json.dumps(record,indent=2)+'\n')
    assert p.returncode==expected,(name,p.stdout[-1000:],p.stderr[-2000:])
    return record

def main():
    OUT.mkdir(parents=True,exist_ok=True);TARGET.mkdir(parents=True,exist_ok=True)
    env=os.environ.copy();env['CARGO_TARGET_DIR']=str(TARGET/'runtime')
    run('runtime-build',['cargo','build','--offline','--release','-p','chr-compiled','--no-default-features','--features','experiment','--lib','--bin','chr-access-emit'],4<<30,env)
    release=TARGET/'runtime/release';rlib=release/'libchr_compiled.rlib'
    shared={str(p.relative_to(ROOT)):digest(p) for p in sorted((release/'deps').glob('*.rlib'))}
    shared[str(rlib.relative_to(ROOT))]=digest(rlib)
    artifacts={}
    for family in ['generic','chain','payload','subscription','dispatch16']:
        source=TARGET/(family+'.rs');binary=TARGET/family
        generated=run('emit-'+family,[release/'chr-access-emit',family,source],1<<30)
        compilation=run('compile-'+family,['rustc','--edition','2024','-C','opt-level=3','-C','codegen-units=1',source,'--extern','chr_compiled='+str(rlib),'-L','dependency='+str(release/'deps'),'-o',binary],4<<30)
        assert all(digest(ROOT/p)==h for p,h in shared.items()),'shared runtime changed during artifact compilation'
        artifacts[family]=dict(source_sha256=digest(source),binary_sha256=digest(binary),source_bytes=source.stat().st_size,binary_bytes=binary.stat().st_size)
        print('compiled',family,flush=True)
    checked=0
    for family in ['chain','payload','subscription','dispatch16']:
        for mode in ['generic','planned','specialized','native','native-generic-repair','native-planned','native-specialized']:
            for size in [0,5]:
                binary=TARGET/(family if mode.startswith('native') else 'generic')
                result=run(f'run-{family}-{mode}-{size}',[binary,mode,family,size,3],1<<30)
                rows=[json.loads(line) for line in result['stdout'].splitlines()];header=rows[-1];queries=rows[:-1]
                assert len(queries)==3 and [q['query'] for q in queries]==[0,1,2]
                assert [q['size'] for q in queries]==[size,size+1,size]
                assert all(q['validated'] and q['query_ns']==sum(q[k] for k in ['setup_ns','execute_ns','observation_ns','engine_drop_ns','answer_drop_ns']) for q in queries)
                assert header['mode']==mode and header['family']==family and header['queries']==3
                assert header['lifecycle_ns']==sum(q['query_ns'] for q in queries)+sum(header[k] for k in ['source_ns','prepare_ns','prepared_drop_ns'])
                assert header['counters'] is False and header['allocator']=='ordinary'
                checked+=len(queries)
        print('validated',family,flush=True)
    rejection=run('source-mismatch',[TARGET/'payload','native','chain',0,1],1<<30,expected=2)
    assert not rejection['stdout'] and 'does not match supplied rules' in rejection['stderr']
    assert checked==168
    freeze=dict(parent_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),rustc=subprocess.check_output(['rustc','-Vv'],text=True),shared_rlibs=shared,artifacts=artifacts,complete_queries=checked,successful_processes=56,mismatch_rejections=1,interpretation='accounting/correctness gate only; no ranking or timing inference')
    (OUT/'artifacts.json').write_text(json.dumps(freeze,indent=2)+'\n')
    print('gate complete:',checked,'queries; shared runtime unchanged',flush=True)
if __name__=='__main__':main()
