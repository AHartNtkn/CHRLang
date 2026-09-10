import json,pathlib,re,shutil,subprocess,tempfile,tomllib,resource
root=pathlib.Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s02-read-near-miss'
with tempfile.TemporaryDirectory(prefix='chr-read-mutation-') as temp:
 base=pathlib.Path(temp);p=base/'research/chr-relational';p.parent.mkdir()
 shutil.copytree(root/'research/chr-relational',p)
 (p.parent/'chr-direct-conditional').symlink_to(root/'research/chr-direct-conditional',target_is_directory=True)
 # Standalone copy uses the same dependencies and toolchain; only candidate lookup is mutated.
 manifest=p/'Cargo.toml';s=manifest.read_text();package=tomllib.loads((root/'Cargo.toml').read_text())['workspace']['package']
 for key in ['version','edition']:s=s.replace(f'{key}.workspace = true',f'{key} = "{package[key]}"')
 s=re.sub(r'path = "([^\"]+)"',lambda m:'path = '+json.dumps(str((root/'research/chr-relational'/m.group(1)).resolve())) if m.group(1).startswith('../') else m.group(0),s)
 manifest.write_text(s+'\n[workspace]\n')
 code=p/'src/contextual.rs';s=code.read_text();old='.set(arena.work.candidates.get() + 1)';assert old in s
 s=s.replace(old,'.set(arena.work.candidates.get())',1);code.write_text(s)
 command=['cargo','test','--manifest-path',str(manifest),'--test','read_near_miss','--features','deduction-work','--no-run','--message-format=json','--target-dir',str(root/'target/s02-read-near-mutation')]
 result=subprocess.run(command,capture_output=True,text=True,timeout=300);(out/'mutation-build.log').write_text(result.stderr);assert result.returncode==0,result.stderr
 binary=next(json.loads(x)['executable'] for x in result.stdout.splitlines() if json.loads(x).get('executable'))
 def limits():resource.setrlimit(resource.RLIMIT_CPU,(120,120));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
 result=subprocess.run([binary,'same_inputs_examine_each_prior_incompatible_context','--nocapture'],capture_output=True,text=True,timeout=120,preexec_fn=limits)
 (out/'mutation.log').write_text(result.stdout+result.stderr)
 assert result.returncode!=0 and 'must inspect all incompatible prior target records' in result.stderr,(result.returncode,result.stdout,result.stderr)
 print('PASS: candidate-count omission rejected by exact near-miss operation witness.')
