"""Verify source opportunity and split-process owner receipts, not a cost matrix."""
from pathlib import Path
import hashlib,importlib.util,json,sys
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s08-continuing-qualification'
def read(name):return json.loads((RAW/name).read_text())
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
if __name__=='__main__':
    for name in ['manifest.json','split-manifest.json']:
        for path,h in read(name)['sha256'].items():assert digest(ROOT/path)==h,path
    configs=read('manifest.json')['configs'];assert len(configs)==len({tuple(c) for c in configs})==8
    counts=[]
    for i,(mode,source) in enumerate(configs):
        r=read(f'gate-{i}.json');assert r['command'][1:]==[mode,source]
        assert r['returncode']==0 and not r['stderr']
        rows=[json.loads(line) for line in r['stdout'].splitlines()]
        assert rows[-1]=={'validated':True};assert [x['answers'] for x in rows[:-1]]==[1,8,32,64]
        for row in rows[:-1]:
            expected=row['answers'] if source=='resource' and mode.endswith('-reclaim') else 0
            assert row['removed_results']==row['removed_claims']==expected
        counts.append(dict(mode=mode,source=source,**rows[-2]))
    config=read('split-manifest.json')['config']
    for phase in ['validate','measure']:
        r=read(f'split-{phase}.json');assert r['command'][1:]==list(map(str,config))+[phase]
        assert r['returncode']==0 and not r['stderr']
    assert json.loads(read('split-validate.json')['stdout'])=={'validated':True}
    row=json.loads(read('split-measure.json')['stdout'])
    spec=importlib.util.spec_from_file_location('ownership',ROOT/'research/chr-reuse/experiments/continuing_ownership.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);m.validate(row,config)
    ss=row['snapshots'];engine=ss[-4]['memory']['live_end']-ss[-3]['memory']['live_end'];consumer=ss[-3]['memory']['live_end']-ss[-2]['memory']['live_end']
    audit=dict(gate_processes=8,gate_answers=512,independent_replay_answers=512,allocation_qualification_processes=1,comparative_processes=0,counts=counts,engine_delivery_bytes=engine,consumer_bytes=consumer,root_restored=True,retrospective_receipt_sha256={p.name:digest(p) for p in sorted(RAW.iterdir()) if p.is_file() and p.name!='audit.json'})
    (RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n');print(json.dumps(audit))
