"""Isolated semantic evidence with exclusive records and pre-run source freeze."""
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import signal
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'docs/experiments/results'
PREFIX = 'E18-relational-gate-v3'

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def reserve(paths):
    if any(p.exists() for p in paths):
        raise FileExistsError('recorded gate exists; use a separately registered version')
    # Exclusive creation also protects against concurrent launches.
    return [p.open('x') for p in paths]

def bound():
    resource.setrlimit(resource.RLIMIT_AS, (1073741824,1073741824))

def main():
    paths = [ROOT/'docs/experiments/registrations/E18-relational-gate.md']
    paths += sorted((ROOT/'research/chr-relational').glob('*.py'))
    frozen = {str(p.relative_to(ROOT)):digest(p) for p in paths}
    order = [{'phase':phase,'offset':offset} for phase in ['gate','replay'] for offset in range(0,432,4)]
    manifest_path=OUT/f'{PREFIX}-manifest.json'
    records_path=OUT/f'{PREFIX}.jsonl'
    manifest, records=reserve([manifest_path,records_path])
    with manifest, records:
        json.dump({'purpose':'finite semantic gate, not comparative costs','sha256':frozen,
                   'order':order,'timeout_seconds':30,'address_space_bytes':1073741824,
                   'python':sys.version,'platform':platform.platform(),
                   'created_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())},manifest,indent=2)
        manifest.write('\n');manifest.flush();os.fsync(manifest.fileno())
        for cell in order:
            assert all(digest(ROOT/name)==value for name,value in frozen.items()), 'frozen input changed'
            command=[sys.executable,'research/chr-relational/gate.py','--batch',str(cell['offset'])]
            start=time.monotonic()
            child=subprocess.Popen(command,cwd=ROOT,stdout=subprocess.PIPE,stderr=subprocess.PIPE,
                                   text=True,preexec_fn=bound,start_new_session=True)
            try:
                stdout,stderr=child.communicate(timeout=30)
                status=child.returncode
            except subprocess.TimeoutExpired:
                os.killpg(child.pid,signal.SIGKILL)
                stdout,stderr=child.communicate();status='timeout'
            row={**cell,'exit':status,'stdout':stdout,'stderr':stderr,'wall_seconds':time.monotonic()-start}
            if status==0:
                try:
                    row['result']=json.loads(stdout)
                    assert row['result']['offset']==cell['offset']
                except (ValueError,AssertionError) as error:
                    row['parse_error']=str(error)
            records.write(json.dumps(row,sort_keys=True)+'\n');records.flush();os.fsync(records.fileno())
            if status!=0 or 'parse_error' in row:
                raise RuntimeError(f'preserved unsuccessful batch: {cell}')
        assert all(digest(ROOT/name)==value for name,value in frozen.items()), 'frozen input changed'
    print('216 isolated semantic batches recorded')

if __name__=='__main__':
    main()
