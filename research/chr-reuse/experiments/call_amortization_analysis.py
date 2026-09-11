"""Audit the registered campaign coverage and attribute its complete costs."""
import collections,gzip,hashlib,itertools,json,random,statistics,sys
sys.dont_write_bytecode=True
import call_amortization as run

def phases_match(receipt,cell,metered):
    _,family,_,queries,_=cell
    rows=iter(json.loads(x)for x in receipt['stdout'].splitlines()[1:])
    expected=[('source',0),('prepare',0),('source_dispose',0)]
    for q in range(queries):
        expected.extend([('input',q),('setup',q)])
        expected.extend((phase,q)for _ in range(1 if family==1 else 2)for phase in ['service_observe','consume'])
        expected.extend((phase,q)for phase in ['service_observe','engine_dispose','input_dispose'])
    expected.extend([('prepared_dispose',queries),('consumer_dispose',queries)])
    for phase,q in expected:
        row=next(rows);assert (row['phase'],row['query'])==(phase,q)
        assert (row['ns']is None)==metered and (row['memory']is not None)==metered
    assert next(rows,None)is None

def audit_emitter():
    raw=run.RAW;record=json.loads((raw/'emitter-correspondence.json').read_text())
    assert record['build']['returncode']==0
    assert run.own.sha(raw/'registered/research/chr-reuse/examples/call_program_emit.rs')==record['source_sha256']
    for tool in record['tools'].values():assert run.own.sha(run.Path(tool['path']))==tool['sha256']
    seen=set()
    for check in record['checks']:
        command=check['command'];kind=next(k for k,v in record['tools'].items()if v['path']==command[0])
        mode,family=command[1],int(command[2]);assert mode in run.SOURCES and family in range(4)
        key=kind,mode,family;assert key not in seen;seen.add(key)
        assert check['returncode']==0 and not check['stderr']
        path=run.Path(command[3]);assert run.own.sha(path)==check['source_sha256']==run.own.sha(run.ROOT/check['expected'])
        assert json.loads(check['stdout'])['source_bytes']==path.stat().st_size
        for mode_kind,reps in [('primary',5),('meter',1)]:
            for rep in range(reps):assert run.own.sha(raw/'sources'/(run.stem(mode_kind,family,mode,rep)+'.rs'))==check['source_sha256']
    assert len(seen)==16

def analyze():
    audit_emitter()
    result=run.audit();raw=run.RAW
    assert result==json.loads((raw/'analysis.json').read_text())
    freeze=json.loads((raw/'freeze.json').read_text())
    scenarios=json.loads((raw/'scenarios.json').read_text())
    initial=[list((f,n,q,p))for f,n,p in run.REGIMES for q in [64,8192]]
    assert scenarios[:8]==initial and len(scenarios)in [8,9]
    expected=set();compile_rows=[]
    for kind,reps in [('primary',5),('meter',1)]:
        for f,source,rep in itertools.product(range(4),run.SOURCES,range(reps)):
            name=run.stem(kind,f,source,rep);src=run.TARGET/(name+'.rs');binary=run.TARGET/name
            expected.update(prefix+name for prefix in ['emit-','compile-','preflight-'])
            e=run.read('emit-'+name);c=run.read('compile-'+name);p=run.read('preflight-'+name)
            assert e['command']==list(map(str,[run.LIB/'primary/release/examples/call_program_emit',source,f,src]))
            cmd=['rustc','--edition','2024','-C','opt-level=3','-C','codegen-units=1','-C','lto=off',src,'-L','dependency='+str(run.LIB/kind/'release/deps'),'-o',binary]
            for crate in ['chr_reuse','chr_compiled','chr_syntax','chr_observe']:cmd+=['--extern',crate+'='+str(run.LIB/kind/'release'/('lib'+crate+'.rlib'))]
            if kind=='meter':cmd+=['--cfg','feature="alloc-meter"']
            assert c['command']==list(map(str,cmd))
            n,policy=next((n,policy)for family,n,policy in run.REGIMES if family==f)
            cell=('generated'if source=='generated'else'direct',f,n,4,policy)
            assert p['command']==list(map(str,[binary,*run.args(cell)]));run.validate(p,cell,kind=='meter');phases_match(p,cell,kind=='meter')
            emission=json.loads(e['stdout']);assert emission['source_bytes']==(raw/'sources'/src.name).stat().st_size
            if kind=='primary':compile_rows.append(dict(family=f,source=source,rep=rep,compile_ns=c['wall_ns'],cpu_seconds=c['cpu_seconds'],**emission,**freeze['artifacts'][name]))
    artifacts={run.stem(k,f,s,r)for k,reps in [('primary',5),('meter',1)]for f,s,r in itertools.product(range(4),run.SOURCES,range(reps))}
    assert set(freeze['artifacts'])==artifacts
    disposal=json.loads((raw/'disposal.json').read_text());assert set(disposal)==artifacts and all(v>0 for v in disposal.values())
    assert run.own.sha(run.Path(freeze['clock']['path']))==freeze['clock']['sha256']
    for i in range(3):
        expected.add(f'clock-{i}');assert run.read(f'clock-{i}')['command']==[freeze['clock']['path'],'clock-check']
    rng=random.Random(607099)
    for start,end in [(0,8)]+([(8,9)]if len(scenarios)==9 else[]):
        schedule=[]
        for k in range(5):
            order=list(range(start,end));rng.shuffle(order)
            for si in order:
                modes=run.MODES.copy();rng.shuffle(modes)
                schedule.extend([si,k,m]for m in modes)
        assert json.loads((raw/f'schedule-{start}.json').read_text())==schedule
    rows=[]
    for si,(f,n,q,p)in enumerate(scenarios):
        for mode in run.MODES:
            cell=(mode,f,n,q,p);names=[(f'alloc-{si}-{k}-{mode}','meter')for k in range(2)]+[(f'check-{si}-{mode}','primary')]+[(f'time-{si}-{k}-{mode}','primary')for k in range(5)]
            for name,kind in names:
                expected.add(name);receipt=run.read(name);assert receipt['command']==list(map(str,[run.binary(kind,f,mode),*run.args(cell)]));phases_match(receipt,cell,kind=='meter')
            phases=collections.defaultdict(list);session_totals=[]
            for k in range(5):
                totals=collections.Counter()
                for line in run.read(f'time-{si}-{k}-{mode}')['stdout'].splitlines()[1:]:
                    entry=json.loads(line);totals[entry['phase']]+=entry['ns']
                session_totals.append(sum(totals.values()))
                for phase,value in totals.items():phases[phase].append(value)
            allocation=run.validate(run.read(f'alloc-{si}-0-{mode}'),cell,True)
            retained=allocation.pop('query_retained')
            allocation['query_retained']={'first':retained[0],'last':retained[-1],'min':min(retained),'max':max(retained)}
            rows.append(dict(scenario=[f,n,q,p],mode=mode,median_ns=statistics.median(session_totals),phases={k:statistics.median(v)for k,v in phases.items()},allocation=allocation))
    receipts=json.loads((raw/'receipts.json').read_text());assert set(receipts)==expected
    preliminary=run.comparisons(scenarios[:8],{});assert preliminary==json.loads((raw/'pre-disposal.json').read_text())
    eligible=[]
    for ri,(f,n,p)in enumerate(run.REGIMES):
        for mi,mode in enumerate(run.MODES[-2:]):
            xs=[x for x in preliminary['comparisons']if x['scenario']==[f,n,8192,p]and x['candidate']==mode and x['control']in ['direct','scan','sealed','planned','planned-sealed']]
            assert len(xs)==5
            if all(x['runtime']['median']<=.9 for x in xs)and not any(x['installed']['status']=='gain'for x in xs):eligible.append([min(x['control_ns']-x['candidate_ns']for x in xs),-ri,-mi])
    selection=None
    if eligible:
        score,ri,mi=max(eligible);f,n,p=run.REGIMES[-ri]
        selection=dict(source_index=-ri,mode=run.MODES[-2:][-mi],minimum_saving_ns=score)
        assert scenarios[-1]==[f,n,32768,p]and len(scenarios)==9
    else:assert len(scenarios)==8
    assert json.loads((raw/'selection.json').read_text())==dict(eligible=eligible,selected=selection)
    before={(tuple(x['scenario']),x['candidate'],x['control']):x['installed']['status']for x in preliminary['comparisons']}
    changes=[x for x in result['comparisons']if (tuple(x['scenario']),x['candidate'],x['control'])in before and x['installed']['status']!=before[tuple(x['scenario']),x['candidate'],x['control']]]
    old=run.ORIGINAL;interruption=json.loads((old/'interruption.json').read_text());manifest=json.loads((old/'receipts.json').read_text());assert interruption['terminal_status']==130 and interruption['completed_receipts']==404 and interruption['completed_timing_receipts']==86
    for name,h in manifest.items():
        enc=(old/(name+'.json.gz')).read_bytes();data=gzip.decompress(enc)
        assert hashlib.sha256(enc).hexdigest()==h['gzip_sha256']and hashlib.sha256(data).hexdigest()==h['raw_sha256']
        assert json.loads(data)['returncode']==0
    assert len(manifest)==404 and sum(k.startswith('time-')for k in manifest)==86
    assert len(json.loads((old/'disposal.json').read_text()))==48
    return dict(processes=len(receipts),interrupted_processes=len(manifest),query_sessions=sum(s[2]*len(run.MODES)*8 for s in scenarios),compiler=compile_rows,cells=rows,disposal_qualification_changes=changes)

if __name__=='__main__':
    value=analyze();path=run.RAW/'diagnosis.json'
    if sys.argv[1]=='write':path.write_text(json.dumps(value,indent=2)+'\n')
    else:assert value==json.loads(path.read_text())
    print('Exact command/schedule coverage, source, ownership, interruption and compiler accounting verified:',value['processes'],'processes.')
