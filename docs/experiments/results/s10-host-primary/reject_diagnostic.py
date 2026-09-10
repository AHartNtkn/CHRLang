from pathlib import Path
import importlib.util,json,subprocess,tempfile,sys
root=Path.cwd()
spec=importlib.util.spec_from_file_location('host_gate',root/'research/chr-hvm/host_session/gate.py')
gate=importlib.util.module_from_spec(spec);spec.loader.exec_module(gate)
plan=json.loads((root/'docs/experiments/results/s10-native-prepared/plans.json').read_text())[0]
text=gate.frontend.joined([q['source'] for q in plan['queries']])
budgets=','.join(str(q['limit']) for q in plan['queries'])
with tempfile.TemporaryDirectory(prefix='chr-primary-reject-') as directory:
    p=subprocess.run([sys.executable,str(root/'research/chr-hvm/host_session/run.py'),budgets,directory,str(root/'target/s10-publication-clock/native-diagnostic')],input=text.encode(),capture_output=True,timeout=30,preexec_fn=gate.bounds)
    clean=not list(Path(directory).iterdir())
    assert p.returncode!=0 and p.stdout==b'' and clean
    assert 'primary host requires native serialization clocks disabled' in p.stderr.decode()
    (root/'docs/experiments/results/s10-host-primary/rejection.json').write_text(json.dumps({'returncode':p.returncode,'published_bytes':len(p.stdout),'temporary_root_empty':clean,'stderr':p.stderr.decode()},indent=2)+'\n')
print('Diagnostic clock binary rejected before host publication; temporary artifact cleanup passed')
