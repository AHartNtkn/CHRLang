import subprocess,shutil
from pathlib import Path
root=Path('/home/ahart/Documents/CHRLang')
out=root/'docs/experiments/results/R02-repair-pilot'
for stage,src in [('before',Path('/tmp/chr-r02-repair-before')),('after',root)]:
 for mode,flags in [('time',['--no-default-features','--features','experiment']),('work',['--features','experiment']),('memory',['--no-default-features','--features','alloc-meter'])]:
  target=f'/tmp/chr-r02-repair-{stage}-{mode}'
  cmd=['cargo','build','--release','-p','chr-integrated','--target-dir',target,*flags]
  with (out/f'{stage}-{mode}-build.log').open('w') as log: subprocess.run(cmd,cwd=src,stdout=log,stderr=subprocess.STDOUT,check=True)
  dest=Path('/tmp/chr-r02-repair-bins')/stage/mode;dest.parent.mkdir(parents=True,exist_ok=True)
  shutil.copy2(Path(target)/'release/chr-integrated-cost',dest)
  with (out/f'{stage}-{mode}-features.log').open('w') as log: subprocess.run(['cargo','tree','-p','chr-integrated','-e','features',*flags],cwd=src,stdout=log,stderr=subprocess.STDOUT,check=True)
  print(stage,mode,'built',flush=True)
