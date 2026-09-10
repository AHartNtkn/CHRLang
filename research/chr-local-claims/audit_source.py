"""Replay source outcomes against preserved independent and native controls."""
from pathlib import Path
import hashlib,importlib.util,json,sys
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2];RAW=ROOT/'docs/experiments/results/s03-descriptor-source'
spec=importlib.util.spec_from_file_location('source_run',Path(__file__).with_name('run_source.py'));run=importlib.util.module_from_spec(spec);spec.loader.exec_module(run)
from source import Ref
for attempt in range(1,6):
    directory=RAW if attempt==1 else RAW/f'attempt-{attempt}'
    for name,h in json.loads((directory/'manifest.json').read_text()).items():
        p=ROOT/name
        if p.name=='source.py' and attempt<5:p=RAW/f'attempt-{max(2,attempt)}-source.py'
        if p.name=='run_source.py' and attempt<5:p=RAW/f'attempt-{attempt}-run_source.py'
        assert hashlib.sha256(p.read_bytes()).hexdigest()==h,(attempt,name)
for name,h in json.loads((RAW/'native-manifest.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
sources={s['name']:s for s in json.loads((RAW/'cases.json').read_text())}
native={x['case']:x for x in json.loads((RAW/'native-controls.json').read_text())}
refs={x['case']:x for x in json.loads((RAW/'attempt-5/references.json').read_text())}
r=json.loads((RAW/'attempt-5/results.json').read_text());assert r['status']=='passed' and len(r['rows'])==632
keys=set();snapshots=retries=cancelled=differences=0
for row in r['rows']:
    key=(row['case'],row['ordered'],row['reverse'],row['cancel']);assert key not in keys;keys.add(key)
    source=sources[key[0]];e=run.Engine(source,*key[1:]);answer=e.run()
    assert answer==row['answer'];assert run.gate.normalize(answer)==run.gate.normalize(source['expected'])==run.gate.normalize(refs[key[0]]['answer'])
    nr=native[key[0]];assert nr['result']['code']==0
    assert run.gate.normalize(nr['answer'])==run.gate.normalize(source['expected'])
    match=run.ordered_normalize(answer)==run.ordered_normalize(nr['answer']);assert match==row['matches_ordered_reference']
    if key[1]:assert match
    assert (len(e.descriptors),e.max_live,e.snapshots,e.scan_retries,e.cancelled)==(row['descriptors'],row['max_live'],row['snapshots'],row['scan_retries'],row['cancelled'])
    assert all(not isinstance(v,Ref) for v in e.cells.values())
    assert all(d.status!='pending' and d.cleaned==len(d.writes) for d in e.descriptors)
    retained=json.loads(json.dumps(answer));del e;assert answer==retained
    snapshots+=row['snapshots'];retries+=row['scan_retries'];cancelled+=row['cancelled'];differences+=not match
assert len(keys)==79*8 and (snapshots,retries,cancelled,differences)==(10688,1980,308,6)
for row in json.loads((RAW/'admission.json').read_text()):
    try:run.Engine(row['source'],True,False,False)
    except ValueError as e:assert str(e)==row['rejection']
    else:raise AssertionError('unsupported source admitted')
print('Verified 632 source runs, 79 reference/native controls, 10688 snapshots, 1980 scan retries, 308 application cancellations, 6 ordered counterexamples and 4 admission boundaries.')
