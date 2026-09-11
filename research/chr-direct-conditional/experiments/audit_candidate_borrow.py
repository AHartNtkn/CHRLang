"""Audit paired outcomes, copy attribution and complete lifecycle costs."""
import collections,gzip,hashlib,json,re,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-candidate-borrow';PARENT=ROOT/'docs/experiments/results/s03-post-control-cost';COPY=ROOT/'docs/experiments/results/s03-candidate-copy'
DEMAND=['birth','birth-miss','birth-miss-template'];EXPLICIT=['scan','indexed','sealed','active-scan','active-indexed','native-scan','native-indexed','active-native-scan','active-native-indexed']
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def events(raw):return [json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{')]
def result(raw):return next(e for e in events(raw) if e.get('event')=='result')
def norm(x,base):
    if isinstance(x,list):return [norm(v,base) for v in x]
    if isinstance(x,dict):return {k:(v-base if k in ['live_start','live_end','peak_live'] else norm(v,base)) for k,v in x.items() if k!='ns'}
    return x
def main():
    f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256'];assert sha(BASE/'jobs.json')==f['jobs_sha256'];assert sha(PARENT/'freeze.json')==f['parent_freeze_sha256'];assert sha(PARENT/'sources.zip')==f['parent_archive_sha256']
    with zipfile.ZipFile(BASE/'sources.zip') as z:
        for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h and sha(ROOT/p)==h,p
    with zipfile.ZipFile(PARENT/'sources.zip') as z:
        for p,h in f['before_source_hashes'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
    for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
    clocks=[]
    for i in range(9):
        r=read(BASE/f'calibration-{i}.json');assert r['exit_code']==0 and not r['stderr']
        if i<6:
            c=json.loads(r['stdout']);assert c['samples']==100000;clocks.append(c['median'])
    floor=100*max(clocks)*24
    parent_cells=read(PARENT/'cells.json');parent_raw={}
    for i,j in enumerate(read(PARENT/'meter-order.json')):
        key=tuple(parent_cells[j['index']])
        if key not in parent_raw:parent_raw[key]=result(read(PARENT/'meter'/f'{i}.json'))
    original_cancel={tuple(c):i for i,c in enumerate(read(ROOT/'docs/experiments/results/s03-post-ownership/cells.json'))}
    copy_cells={tuple(r['cell']):i for i,r in enumerate(read(COPY/'cells.json'))}
    jobs=read(BASE/'jobs.json');groups=collections.defaultdict(list);profiles=collections.defaultdict(list);count=0;bridges=0
    expected_phases=[('source',0),('prepare',0)]+[(p,i) for i in range(4) for p in ['input','setup','execute_observe','engine_drop','consumer']]+[('prepared_drop',0),('consumer_drop',0)]
    with gzip.open(BASE/'runs.jsonl.gz','rt') as stream:
        for i,line in enumerate(stream):
            raw=json.loads(line);j=jobs[i];assert raw['index']==i;count+=1
            fam,n,rev,ret=j['scenario'];mode=j['mode'];kind=j['kind'];cancel=j['cancel'];key=(*j['scenario'],mode,cancel)
            assert raw['command']==[f['binaries'][kind]['path'],mode,fam,str(n),str(rev).lower(),ret,str(cancel).lower()]
            assert raw['first_clock']=='off' and raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'],i
            r=result(raw);meter=kind in ['new-meter','new-profile'];assert r['meter']==meter
            assert [(p['phase'],p['query']) for p in r['phases']]==expected_phases
            assert all(e['first_ns'] is None for e in r['endpoints'])
            assert [e['complete'] for e in r['endpoints']]==([False,True,False,True] if cancel else [True]*4)
            assert all(e['answers']==1 for e in r['endpoints'] if e['complete'])
            if cancel:
                pi=original_cancel[(fam,n,rev,mode,ret,True)]
                prior=result(read(ROOT/'docs/experiments/results/s03-post-ownership/runs'/f'{pi}-meter-0.json'))
            else:prior=parent_raw[(fam,n,rev,ret,mode)]
            assert r['endpoints']==prior['endpoints'],(i,'changed source work')
            if meter:
                ms=[p['reading']['memory'] for p in r['phases']];base=ms[0]['live_start'];assert ms[-1]['live_end']==base
                assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));nr=norm(r,base)
                if mode in EXPLICIT:
                    assert nr==norm(prior,prior['phases'][0]['reading']['memory']['live_start']);bridges+=1
                if kind=='new-profile':
                    pr=next(e for e in events(raw) if e.get('event')=='candidate-profile');profiles[key].append(pr)
                groups[(kind,key)].append(nr)
            elif not j['warmup']:
                assert all(p['reading']['memory'] is None for p in r['phases'])
                phases=collections.Counter()
                for p in r['phases']:phases[p['phase']]+=p['reading']['ns']
                total=sum(phases.values());groups[(kind,key)].append(dict(rep=j['rep'],total_ns=total,phases=dict(phases),sensitive=total<floor))
    assert count==len(jobs)==8136 and bridges==648
    summaries=[];attribution=[]
    for (kind,key),values in sorted(groups.items()):
        fam,n,rev,ret,mode,cancel=key
        if kind in ['new-meter','new-profile']:
            assert all(v==values[0] for v in values)
            assert len(values)==(2 if kind=='new-profile' or mode in DEMAND and not cancel else 1)
            ms=[p['reading']['memory'] for p in values[0]['phases']]
            summaries.append(dict(kind=kind,key=key,requested_bytes=sum(m['requested_bytes'] for m in ms),peak_excess=max(m['peak_live'] for m in ms),phases=values[0]['phases']))
            if kind=='new-profile':
                assert values==groups[('new-meter',key)],key
                assert profiles[key][0]==profiles[key][1]
                ci=copy_cells[(fam,n,rev,mode,ret,False)]
                old=[]
                for rep in [0,1]:
                    raw=read(COPY/'runs'/f'{ci}-{rep}.json');assert raw['exit_code']==0
                    old.append(next(e for e in events(raw) if e.get('event')=='candidate-profile'))
                assert old[0]==old[1]
                prior=parent_raw[(fam,n,rev,ret,mode)]
                old_total=sum(p['reading']['memory']['requested_bytes'] for p in prior['phases'])
                saved=old_total-sum(m['requested_bytes'] for m in ms)
                copy_saved=sum(p['requested_bytes'] for p in old[0]['rows'])-sum(p['requested_bytes'] for p in profiles[key][0]['rows'])
                assert saved==copy_saved,(key,saved,copy_saved)
                assert profiles[key][0]['rows'][0]['requested_bytes']==0
                attribution.append(dict(key=key,old_total=old_total,new_total=sum(m['requested_bytes'] for m in ms),saved_bytes=saved,old_profile=old[0],new_profile=profiles[key][0]))
        else:
            assert len(values)==(1 if cancel else 5)
            summaries.append(dict(kind=kind,key=key,median_ns=statistics.median(v['total_ns'] for v in values),sensitive=any(v['sensitive'] for v in values),samples=values))
    times={(r['kind'],tuple(r['key'])):r for r in summaries if r['kind'] in ['new-time','old-time'] and not r['key'][-1]}
    contrasts=[]
    for (kind,key),left in times.items():
        if kind!='new-time':continue
        for mode in [key[-2],*EXPLICIT]:
            right=times[('old-time',(*key[:-2],mode,False))]
            a={s['rep']:s['total_ns'] for s in left['samples']};b={s['rep']:s['total_ns'] for s in right['samples']};ratios=[a[r]/b[r] for r in sorted(a)]
            contrasts.append(dict(key=key,control=mode,median_ratio=statistics.median(ratios),ratios=ratios,sensitive=left['sensitive'] or right['sensitive']))
    extract=lambda s:re.findall(r'POST_WORK,[^\r\n]+',s)
    work=[extract((BASE/f'work-{i}.log').read_text()) for i in [0,1]]
    parent_work=extract(read(ROOT/'docs/experiments/results/s03-post-ownership/work-1.json')['stdout']);assert work[0]==work[1]==parent_work and len(work[0])==288
    out=dict(processes=count,explicit_allocation_bridges=bridges,new_allocation_pairs=216,profile_pairs=72,exact_work_rows=288,clock_medians_ns=clocks,clock_floor_ns=floor,summary=summaries,attribution=attribution,contrasts=contrasts)
    (BASE/'audit.json').write_text(json.dumps(out,indent=2));print(json.dumps({k:v for k,v in out.items() if k not in ['summary','attribution','contrasts']}))
if __name__=='__main__':main()
