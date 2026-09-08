"""Deterministic runner checks; fake subprocesses only, no CHR execution."""
import copy
import gzip
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import time

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path.cwd() / 'research/chr-compiled/experiments'))
import recursive_lifecycle_pilot as p

order = p.manifest()
assert len(order) == 324
assert len({(r['kind'], r['rep'], tuple(r['cell'])) for r in order}) == 324
assert sum(r['cell'][2] for r in order) == 62316
ops, binaries = p.build_plan(Path('/tmp/plan'))
assert len(ops) == len({op['label'] for op in ops}) == 67
assert p.expected('add', 0, 10, 10) == ([('x', 10), ('y', 11), ('unused', 99)], [])
assert p.expected('add', 1, 10, 10) is None
assert p.expected('fresh', 1, 10, 10) is None
assert p.expected('add', 0, ('a', []), ('clash', [])) is None
assert p.canonical(p.expected('fresh', 2, 10, 11)) == p.canonical(([
    ('x', 1), ('y', ('pair', [('pair', [1, 2]), 3])), ('unused', 4)], []))
root = Path(tempfile.mkdtemp(prefix='t046-lifecycle-selfcheck-'))
r, out, err = p.process([sys.executable, '-c', 'import sys;sys.stdout.buffer.write(bytes([1,0,0,0,1]));sys.stdout.flush()'], Path.cwd(), root/'fake.rss', time.monotonic()+3, session=True)
assert r['status'] == 'complete' and out == bytes([1,0,0,0,1])
assert r['first_response_ns'] <= r['wall_ns']
r, out, err = p.process([sys.executable, '-c', 'import time;print("ready",flush=True);time.sleep(5)'], Path.cwd(), root/'timeout.rss', time.monotonic()+.15, session=True)
assert r['status'] == 'timeout' and b'ready' in out and r['wall_ns'] < 2_000_000_000
r, _, _ = p.process(['x'], root/'missing', root/'spawn.rss', time.monotonic()+1)
assert r['status'] == 'spawn-error'
old = p.selectors.DefaultSelector
p.selectors.DefaultSelector = lambda: (_ for _ in ()).throw(OSError('injected setup error'))
try:
    r, _, _ = p.process([sys.executable, '-c', 'import time;time.sleep(5)'], Path.cwd(), root/'setup.rss', time.monotonic()+1)
finally:
    p.selectors.DefaultSelector = old
assert r['status'] == 'transport-error' and r['exit'] is not None

# A valid first row and explicitly incomplete build set must not become a complete summary.
ops, binaries = p.build_plan(root)
meta = dict(seed=46046, order=order, scratch=str(root), build_operations=ops,
            binaries=binaries, effective_CARGO_INCREMENTAL='0',
            limits=dict(batch_seconds=1200, build_seconds=120, build_bytes=3*1024**3,
                        session_seconds=30, session_bytes=1024**3),
            hardware={'affinity':[0], 'allowed':[0]}, package_manifests={}, frozen={}, artifacts={})
cell = order[0]['cell']
raw = b''
for depth, payload, result, _ in p.requests(cell):
    want = p.expected(cell[0], depth, payload, result)
    encoded = b'\1' if want is None else b'\0' + p.u32(3) + b''.join(p.string(n)+p.term(t) for n,t in want[0]) + p.u32(0)
    raw += p.u32(len(encoded)) + encoded
request = b''.join(p.u32(len(q))+q for *_,q in p.requests(cell))
path = root/'raw.gz'
path.write_bytes(gzip.compress(raw, mtime=0))
binary = binaries[f'{cell[0]}/{cell[3]}']
meta['artifacts'][binary] = 'synthetic'
row = dict(order[0], status='complete', exit=0, wall_ns=10, first_response_ns=5,
           peak_rss_kib=0, preparation_ns=0, stderr='{"preparation_ns":0}',
           command=[binary]+([] if cell[3]=='native' else [cell[0]]), raw_file='raw.gz',
           raw_sha256=hashlib.sha256(raw).hexdigest(), gzip_sha256=p.digest(path),
           request_sha256=hashlib.sha256(request).hexdigest(), observations=p.validate_stream(cell,raw))
p.write_json(root/'metadata.json',meta)
p.write_json(root/'source-check.json',dict(frozen={},artifacts=meta['artifacts'],changed=[],artifacts_changed=[]))
p.write_json(root/'builds.json',[dict(op,status='unattempted') for op in ops])
(root/'sessions.jsonl').write_text(json.dumps(row)+'\n')
a = p.audit(root,True)
assert a['missing']==323 and len(a['failed_builds'])==67 and a['complete_primary_cells']==0
for key,value in [('command',['wrong']),('request_sha256','wrong'),('observations',{}),('rep',99)]:
    bad=copy.deepcopy(row);bad[key]=value
    (root/'sessions.jsonl').write_text(json.dumps(bad)+'\n')
    try:
        p.audit(root,True)
    except AssertionError:
        pass
    else:
        raise AssertionError(key)
print('PASS:324 manifest/62316 responses/67 stages; independent mathematical aliases+occurs/clash; fake framed child, timeout drain, spawn error, setup cleanup; incomplete synthetic audit and4adverse mutations. No CHR engines executed.')
