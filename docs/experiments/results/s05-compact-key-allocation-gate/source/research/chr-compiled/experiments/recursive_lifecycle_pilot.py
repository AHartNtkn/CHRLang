#!/usr/bin/env python3
"""T046 lifecycle launcher/auditor. --plan performs no builds or executions."""
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import selectors
import signal
import statistics
import struct
import subprocess
import tempfile
import tomllib
import sys
import time

ROOT = Path(__file__).resolve().parents[3]
REGISTRATION = ROOT / 'docs/experiments/registrations/R05-recursive-lifecycle.md'
MAX_FRAME = 1 << 20
FAMILIES = ['add', 'fresh']
BACKENDS = ['native', 'direct', 'specialized']
CELLS = [(f, d, reuse, b) for f in FAMILIES for d in [0, 32, 128]
         for reuse in [1, 64, 512] for b in BACKENDS]


def manifest():
    rng = random.Random(46046)
    order = []
    for kind, rep in [('warmup', 0), *[('primary', i) for i in range(5)]]:
        block = [dict(kind=kind, rep=rep, cell=list(c)) for c in CELLS]
        rng.shuffle(block); order.extend(block)
    return order


def build_plan(scratch):
    rng = random.Random(46046)
    generations = [(f, r) for f in FAMILIES for r in range(3)]
    jobs = [('native', f, m, r) for f in FAMILIES for m in ['clean', 'seeded'] for r in range(3)]
    jobs += [(b, 'add', 'clean', r) for b in ['direct', 'specialized'] for r in range(3)]
    rng.shuffle(generations); rng.shuffle(jobs)
    emitter = scratch/'emitter-target/release/examples/recursive_session_packages'
    operations = [dict(label='emitter-install', command=['cargo','build','--offline','--locked','--release','--no-default-features','-p','chr-compiled','--example','recursive_session_packages','--target-dir',str(scratch/'emitter-target')], artifact=str(emitter))]
    for f, r in generations:
        dest = scratch/f'{f}-{r}'
        operations.append(dict(label=f'generate-{f}-{r}', command=[str(emitter),str(dest),f], package_dir=str(dest)))
        for backend in BACKENDS:
            for suffix in ['', '-prereq']:
                manifest_path=dest/(backend+suffix)/'Cargo.toml'
                operations.append(dict(label=f'resolve-{f}-{r}-{backend}{suffix}',command=['cargo','generate-lockfile','--offline','--manifest-path',str(manifest_path)],artifact=str(manifest_path.parent/'Cargo.lock')))
    binaries={}
    for backend, family, mode, rep in jobs:
        dest=scratch/f'{family}-{rep}'
        target=scratch/(f'target-native-{family}-{mode}-{rep}' if backend=='native' else f'target-{backend}-{rep}')
        def compile_op(label, package, fresh):
            return dict(label=label,command=['cargo','build','--offline','--locked','--release','--manifest-path',str(dest/package/'Cargo.toml'),'--target-dir',str(target)],fresh_target=str(target) if fresh else None,artifact=str(target/'release'/('prereq' if package.endswith('-prereq') else 'session')))
        if backend=='native':
            if mode=='seeded':operations.append(compile_op(f'prereq-native-{family}-{rep}','native-prereq',True))
            operations.append(compile_op(f'build-native-{family}-{mode}-{rep}','native',mode=='clean'))
            if mode=='clean' and rep==0:binaries[f'{family}/native']=str(target/'release/session')
        else:
            operations.append(compile_op(f'install-{backend}-{rep}',backend,True))
            if rep==0:
                for f in FAMILIES:binaries[f'{f}/{backend}']=str(target/'release/session')
    return operations,binaries


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write_json(path, value):
    Path(path).write_text(json.dumps(value, indent=2) + '\n')


def u32(n):
    return struct.pack('<I', n)


def string(value):
    data = value.encode('utf-8'); return u32(len(data)) + data


def term(value):
    if isinstance(value, int): return b'\0' + struct.pack('<Q', value)
    name, args = value
    return b'\1' + string(name) + u32(len(args)) + b''.join(term(t) for t in args)


def requests(cell):
    """Owned query shapes; runtime source variables are deliberately shared."""
    family, depth, reuse, _ = cell
    result = []
    for i in range(reuse):
        n = max(0, depth - (i % 2))
        payload, output = [(10, 11), (('a', []), 11), (10, 10), (('a', []), ('clash', []))][i % 4]
        control = ('z', [])
        for _ in range(n): control = ('s', [control])
        encoded = (u32(1) + string(family) + u32(3) + term(control) + term(payload) + term(output)
                   + u32(3) + b''.join(string(name) + struct.pack('<Q', v)
                                      for name, v in [('x', 10), ('y', 11), ('unused', 99)]))
        result.append((n, payload, output, encoded))
    return result


def expected(family, depth, payload, output):
    """Independent finite-tree substitution; no CHR engine/code is consulted."""
    value = payload
    for i in range(depth): value = ('s', [value]) if family == 'add' else ('pair', [value, 1000 + i])
    bindings = {}
    def deref(x):
        while isinstance(x, int) and x in bindings: x = bindings[x]
        return x
    def occurs(var, x):
        todo = [x]
        while todo:
            x = deref(todo.pop())
            if isinstance(x, int):
                if x == var: return True
            else: todo.extend(x[1])
        return False
    pending = [(value, output)]
    while pending:
        a, b = map(deref, pending.pop())
        if a == b: continue
        if isinstance(a, int):
            if occurs(a, b): return None
            bindings[a] = b
        elif isinstance(b, int):
            if occurs(b, a): return None
            bindings[b] = a
        elif a[0] != b[0] or len(a[1]) != len(b[1]): return None
        else: pending.extend(zip(a[1], b[1]))
    def resolve(x):
        x = deref(x)
        return x if isinstance(x, int) else (x[0], [resolve(t) for t in x[1]])
    return [(name, resolve(v)) for name, v in [('x', 10), ('y', 11), ('unused', 99)]], []


def canonical(answer):
    variables = {}
    def walk(t):
        if isinstance(t, int): return ('var', variables.setdefault(t, len(variables)))
        return (t[0], tuple(walk(a) for a in t[1]))
    outputs, residual = answer
    return tuple((n, walk(t)) for n, t in outputs), tuple((n, tuple(walk(t) for t in ts)) for n, ts in residual)


class Decoder:
    def __init__(self, data): self.data, self.pos, self.nodes = data, 0, 0
    def take(self, n):
        assert n <= len(self.data) - self.pos, 'truncated payload'
        data = self.data[self.pos:self.pos+n]; self.pos += n; return data
    def number(self, n=4): return int.from_bytes(self.take(n), 'little')
    def string(self): return self.take(self.number()).decode('utf-8')
    def node(self):
        self.nodes += 1; assert self.nodes <= 100000
    def count(self):
        n = self.number(); assert n <= 100000 - self.nodes; return n
    def term(self, depth=1):
        assert depth <= 512; self.node(); tag = self.number(1)
        if tag == 0: return self.number(8)
        assert tag == 1
        name = self.string(); return name, [self.term(depth+1) for _ in range(self.count())]
    def response(self):
        tag = self.number(1)
        if tag == 0:
            outputs = []
            for _ in range(self.count()):
                self.node(); outputs.append((self.string(), self.term()))
            residual = []
            for _ in range(self.count()):
                self.node(); name = self.string(); residual.append((name, [self.term() for _ in range(self.count())]))
            value = outputs, residual
        elif tag == 1: value = None
        else:
            assert tag in [2, 3, 4]; value = self.string()
        assert self.pos == len(self.data), 'trailing response bytes'
        return tag, value


def validate_stream(cell, raw):
    position, successes, failures = 0, 0, 0
    for depth, payload, output, _ in requests(cell):
        assert len(raw)-position >= 4, 'missing response header'
        length = int.from_bytes(raw[position:position+4], 'little'); position += 4
        assert length <= MAX_FRAME and length <= len(raw)-position, 'invalid response frame'
        tag, answer = Decoder(raw[position:position+length]).response(); position += length
        want = expected(cell[0], depth, payload, output)
        if want is None:
            assert tag == 1, ('expected finite failure', tag, answer); failures += 1
        else:
            assert tag == 0, ('non-success outcome', tag, answer)
            assert canonical(answer) == canonical(want), 'complete joint answer mismatch'; successes += 1
    assert position == len(raw), 'extra responses'
    return dict(successes=successes, failures=failures, responses=successes+failures)


def hardware():
    allowed = sorted(os.sched_getaffinity(0)); assert 0 in allowed
    base = Path('/sys/devices/system/cpu/cpu0/topology')
    topo = {k: (base/k).read_text().strip() for k in ['core_id', 'physical_package_id', 'thread_siblings_list']}
    rel = Path('/proc/self/cgroup').read_text().strip().split('::', 1)[1]
    cur = Path('/sys/fs/cgroup'+rel); root = Path('/sys/fs/cgroup'); quotas = {}
    while True:
        row = {k: (cur/k).read_text().strip() if (cur/k).exists() else None for k in ['cpu.max', 'memory.max', 'cpuset.cpus.effective']}
        if row['cpu.max'] and not row['cpu.max'].startswith('max '):
            quota, period = map(int, row['cpu.max'].split()); assert quota >= period
        quotas[str(cur)] = row
        if cur == root: break
        assert root in cur.parents; cur = cur.parent
    return dict(allowed=allowed, affinity=[0], topology=topo, ancestors=quotas)


def bounded(memory, cpu):
    def apply():
        os.sched_setaffinity(0, {0})
        resource.setrlimit(resource.RLIMIT_AS, (memory, memory))
        resource.setrlimit(resource.RLIMIT_CPU, (cpu, cpu))
    return apply


def process(command, cwd, rssfile, deadline, data=b'', session=False):
    """Full-duplex pump; wall deadline includes spawn, plus at most 1s killed-pipe drain."""
    timeout = min(30 if session else 120, max(0, deadline-time.monotonic()))
    if timeout <= 0: return dict(status='unattempted', reason='batch-budget'), b'', b''
    environment = os.environ.copy(); environment['CARGO_INCREMENTAL'] = '0'
    began_monotonic = time.monotonic(); begin = time.perf_counter_ns()
    try:
        child = subprocess.Popen(['/usr/bin/time', '-f', '%M', '-o', str(rssfile), *command], cwd=cwd,
                             stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             env=environment, start_new_session=True, preexec_fn=bounded((1 if session else 3)*1024**3, 30 if session else 120))
    except (OSError, subprocess.SubprocessError) as error:
        return dict(status='spawn-error',error=str(error),command=command,cwd=str(cwd),wall_ns=time.perf_counter_ns()-begin), b'', b''
    selector = None; streams = {'stdout': bytearray(), 'stderr': bytearray()}
    sent, first, status = 0, None, 'complete'; end = began_monotonic+timeout
    try:
        selector = selectors.DefaultSelector()
        for name, pipe in [('stdout', child.stdout), ('stderr', child.stderr)]:
            os.set_blocking(pipe.fileno(), False); selector.register(pipe, selectors.EVENT_READ, name)
        if data:
            os.set_blocking(child.stdin.fileno(), False); selector.register(child.stdin, selectors.EVENT_WRITE, 'stdin')
        else: child.stdin.close()
        while selector.get_map():
            if time.monotonic() >= end:
                if status == 'timeout': break
                status = 'timeout'; os.killpg(child.pid, signal.SIGKILL); end = time.monotonic()+1
            for key, _ in selector.select(min(0.05, max(0, end-time.monotonic()))):
                if key.data == 'stdin':
                    try: sent += os.write(key.fd, data[sent:sent+65536])
                    except BrokenPipeError: sent = len(data)
                    if sent == len(data): selector.unregister(key.fileobj); key.fileobj.close()
                else:
                    chunk = os.read(key.fd, 65536)
                    if not chunk: selector.unregister(key.fileobj); key.fileobj.close(); continue
                    streams[key.data].extend(chunk)
                    if session and key.data == 'stdout' and first is None:
                        raw = streams['stdout']
                        if len(raw) >= 4 and len(raw) >= 4+int.from_bytes(raw[:4], 'little'):
                            first = time.perf_counter_ns()-begin
        try: child.wait(timeout=max(.1, end-time.monotonic()))
        except subprocess.TimeoutExpired:
            status = 'timeout'; os.killpg(child.pid, signal.SIGKILL); child.wait()
    except (OSError, ValueError) as error:
        status = 'transport-error'
        try: os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError: pass
        child.wait()
        streams['stderr'].extend(('\ntransport error: '+str(error)).encode())
    finally:
        if selector is not None: selector.close()
        for pipe in [child.stdin, child.stdout, child.stderr]: pipe.close()
    wall = time.perf_counter_ns()-begin
    rss = rssfile.read_text().strip().splitlines() if rssfile.exists() else []
    peak = int(rss[-1]) if rss and rss[-1].isdigit() else None
    if child.returncode != 0 and status == 'complete': status = 'process-error'
    return dict(status=status, exit=child.returncode, wall_ns=wall, first_response_ns=first,
                peak_rss_kib=peak, command=command, cwd=str(cwd)), bytes(streams['stdout']), bytes(streams['stderr'])


def source_freeze():
    paths = {ROOT/'Cargo.toml', ROOT/'Cargo.lock', Path(__file__).resolve(), REGISTRATION}
    for folder in ['research/chr-compiled', 'research/chr-persistent', 'research/chr-observe', 'research/chr-cases', 'crates/chr-syntax', 'crates/chr-programs']:
        paths.update(p for p in (ROOT/folder).rglob('*') if p.is_file() and p.suffix in ['.rs', '.toml'] and 'target' not in p.relative_to(ROOT).parts)
    return {str(p): digest(p) for p in sorted(paths)}


def changed(frozen):
    return [p for p, sha in frozen.items() if not Path(p).is_file() or digest(p) != sha]


def launch(out):
    entered = time.monotonic(); deadline = entered+1200
    assert not out.exists(), 'refuse to overwrite evidence'
    hw = hardware(); frozen = source_freeze(); out.mkdir(parents=True)
    scratch = Path(tempfile.mkdtemp(prefix='chr-recursive-lifecycle-'))
    meta = dict(schema_version=1, seed=46046, order=manifest(), frozen=frozen, scratch=str(scratch),
                hardware=hw, rustc=subprocess.check_output(['rustc', '-Vv'], text=True),
                cargo=subprocess.check_output(['cargo', '-V'], text=True),
                time_version=subprocess.check_output(['/usr/bin/time', '--version'], text=True),
                created_utc=time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
                package_manifests={}, limits=dict(batch_seconds=1200, build_seconds=120, build_bytes=3*1024**3,
                            session_seconds=30, session_bytes=1024**3), artifacts={})
    meta['build_operations'], meta['binaries'] = build_plan(scratch)
    meta['build_environment'] = {k: v for k, v in os.environ.items() if k in ['RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_INCREMENTAL'] or k.startswith('CARGO_PROFILE_RELEASE_')}
    meta['effective_CARGO_INCREMENTAL'] = '0'
    # Refuse profile overrides that would invalidate the registered artifact condition.
    for key, required in [('CARGO_PROFILE_RELEASE_OPT_LEVEL', '3'), ('CARGO_PROFILE_RELEASE_CODEGEN_UNITS', '1'), ('CARGO_PROFILE_RELEASE_INCREMENTAL', 'false')]:
        assert key not in os.environ or os.environ[key] == required, (key, required)
    write_json(out/'metadata.json', meta)
    receipts = []; stopped = None
    try:
        for operation in meta['build_operations']:
            label=operation['label']; row=dict(operation)
            if stopped:
                result, stdout, stderr = dict(status='unattempted',reason=stopped),b'',b''
            else:
                if operation.get('fresh_target'): assert not Path(operation['fresh_target']).exists()
                if label.startswith('build-native-') and '-seeded-' in label:
                    assert not Path(operation['artifact']).exists(), 'seeded target contains native artifact'
                result,stdout,stderr=process(operation['command'],ROOT,out/(label+'.rss'),deadline)
            row.update(result,stdout=stdout.decode(errors='replace'),stderr=stderr.decode(errors='replace'))
            if row['status']=='complete':
                try:
                    if 'package_dir' in operation:
                        phases=json.loads(row['stdout']);row['phases']=phases
                        assert all(type(phases[k]) is int and phases[k]>=0 for k in ['fixture_ns','certificate_ns','emit_ns','package_write_ns','total_ns'])
                        assert sum(phases[k] for k in ['fixture_ns','certificate_ns','emit_ns','package_write_ns'])<=phases['total_ns']<=row['wall_ns']
                        for path in Path(operation['package_dir']).rglob('*'):
                            if path.is_file():
                                meta['artifacts'][str(path)]=digest(path)
                                if path.name=='Cargo.toml':meta['package_manifests'][str(path)]=path.read_text()
                    if 'artifact' in operation:meta['artifacts'][operation['artifact']]=digest(operation['artifact'])
                except (AssertionError,ValueError,KeyError,OSError) as error:
                    row.update(status='validation-error',error=str(error) or repr(error))
            if row['status']!='complete':stopped=stopped or row['status']
            receipts.append(row);write_json(out/'builds.json',receipts);write_json(out/'metadata.json',meta)
            print(label,row['status'],flush=True)
        if not stopped:
            for rep in range(3):
                for backend in ['direct','specialized']:
                    for source in ['Cargo.toml','src/main.rs']:
                        assert (scratch/f'add-{rep}'/backend/source).read_bytes()==(scratch/f'fresh-{rep}'/backend/source).read_bytes()
        assert not changed(frozen)
        with (out/'sessions.jsonl').open('w') as log:
            for index,item in enumerate(meta['order']):
                row = dict(item)
                if stopped or time.monotonic() >= deadline:
                    stopped = stopped or 'batch-budget'; row.update(status='unattempted', reason=stopped)
                else:
                    family, _, _, backend = item['cell']; batch = requests(item['cell'])
                    data = b''.join(u32(len(q))+q for *_,q in batch)
                    binary = Path(meta['binaries'][f'{family}/{backend}'])
                    result, raw, stderr = process([str(binary)]+([] if backend=='native' else [family]), ROOT,
                                                  out/f'session-{index}.rss', deadline, data, True)
                    row.update(result); row['stderr'] = stderr.decode(errors='replace')
                    raw_path = out/f'session-{index}.bin.gz'; raw_path.write_bytes(gzip.compress(raw,mtime=0))
                    row.update(raw_file=raw_path.name, raw_sha256=hashlib.sha256(raw).hexdigest(), gzip_sha256=digest(raw_path), request_sha256=hashlib.sha256(data).hexdigest())
                    if row['status'] == 'complete':
                        try:
                            row['observations'] = validate_stream(item['cell'],raw)
                            row['preparation_ns'] = json.loads(stderr)['preparation_ns']
                            assert type(row['preparation_ns']) is int and 0 <= row['preparation_ns'] <= row['wall_ns']
                            if backend == 'native': assert row['preparation_ns'] == 0
                        except (AssertionError,ValueError,KeyError,UnicodeError) as error:
                            row.update(status='validation-error', error=str(error) or repr(error))
                    if row['status'] != 'complete': stopped = row['status']
                log.write(json.dumps(row)+'\n'); log.flush()
                if (index+1)%36 == 0: print(f'{index+1}/324 session outcomes',flush=True)
    finally:
        write_json(out/'metadata.json',meta); write_json(out/'builds.json',receipts)
        write_json(out/'source-check.json',dict(changed=changed(frozen),frozen=frozen,artifacts_changed=changed(meta['artifacts']),artifacts=meta['artifacts']))


def audit(out, archived=False):
    meta = json.loads((out/'metadata.json').read_text()); assert meta['order'] == manifest() and meta['seed']==46046
    assert meta['effective_CARGO_INCREMENTAL']=='0'
    assert meta['limits']==dict(batch_seconds=1200,build_seconds=120,build_bytes=3*1024**3,session_seconds=30,session_bytes=1024**3)
    assert meta['hardware']['affinity']==[0] and 0 in meta['hardware']['allowed']
    for path, contents in meta['package_manifests'].items():
        assert hashlib.sha256(contents.encode()).hexdigest()==meta['artifacts'][path]
        config=tomllib.loads(contents)
        assert config['profile']['release']=={'opt-level':3,'codegen-units':1,'incremental':False}
        dependencies=config['dependencies']; native=Path(path).parent.name.startswith('native')
        assert set(dependencies)=={'chr-syntax','chr-persistent' if native else 'chr-compiled'}
        assert all(d['default-features'] is False and 'path' in d and set(d)=={'path','default-features'} for d in dependencies.values())
    check=json.loads((out/'source-check.json').read_text())
    assert check['frozen']==meta['frozen'] and check['artifacts']==meta['artifacts']
    assert not check['changed'] and not check['artifacts_changed']
    if not archived:
        assert set(meta['frozen'])==set(source_freeze()), 'source freeze inventory incomplete'
        assert not changed(meta['frozen']) and not changed(meta['artifacts'])
    rows=[json.loads(line) for line in (out/'sessions.jsonl').read_text().splitlines()] if (out/'sessions.jsonl').exists() else []
    assert len(rows)<=324
    failures=[]
    for i,row in enumerate(rows):
        assert all(row[k]==meta['order'][i][k] for k in ['kind','rep','cell'])
        if 'raw_file' in row:
            path=out/row['raw_file'];assert digest(path)==row['gzip_sha256'];raw=gzip.decompress(path.read_bytes());assert hashlib.sha256(raw).hexdigest()==row['raw_sha256']
            data=b''.join(u32(len(q))+q for *_,q in requests(row['cell']));assert hashlib.sha256(data).hexdigest()==row['request_sha256']
        if row['status']!='complete': failures.append(i);continue
        assert row['exit']==0 and row['peak_rss_kib'] is not None and row['peak_rss_kib']>=0
        assert 0<=row['preparation_ns']<=row['first_response_ns']<=row['wall_ns']
        assert json.loads(row['stderr'])['preparation_ns']==row['preparation_ns']
        assert validate_stream(row['cell'],raw)==row['observations']
        family, _, _, backend=row['cell']
        assert row['command']==[meta['binaries'][f'{family}/{backend}']]+([] if backend=='native' else [family])
        assert row['command'][0] in meta['artifacts']
        if backend=='native':assert row['preparation_ns']==0
    builds=json.loads((out/'builds.json').read_text());labels=[r['label'] for r in builds]
    assert (meta['build_operations'],meta['binaries']) == build_plan(Path(meta['scratch']))
    build_order=[r['label'] for r in meta['build_operations']]
    assert labels == build_order[:len(labels)]
    for row,op in zip(builds,meta['build_operations']):
        assert all(row[k]==v for k,v in op.items()), 'build command or condition differs'
    assert len(labels)==len(set(labels))
    for label,count in [('generate-',6),('build-native-',12),('install-',6),('prereq-native-',6),('resolve-',36)]:
        assert sum(x.startswith(label) for x in build_order)==count,(label,labels)
    assert not labels or labels[0]=='emitter-install'
    for row in builds:
        if row['status']!='complete': continue
        assert row['exit']==0 and row['wall_ns']>0 and row['peak_rss_kib'] is not None
        if row['label'].startswith('generate-'):
            p=row['phases'];assert json.loads(row['stdout'])==p
            assert sum(p[k] for k in ['fixture_ns','certificate_ns','emit_ns','package_write_ns'])<=p['total_ns']<=row['wall_ns']
        else:
            assert '--offline' in row['command']
            if not row['label'].startswith('resolve-'):assert '--locked' in row['command'] and '--release' in row['command']
    summary=[]
    for cell in CELLS:
        group=sorted([r for r in rows if r['cell']==list(cell) and r['kind']=='primary' and r['status']=='complete'],key=lambda r:r['rep'])
        entry=dict(cell=list(cell),complete=len(group)==5,samples=[{k:r[k] for k in ['rep','wall_ns','first_response_ns','peak_rss_kib','preparation_ns']} for r in group])
        if len(group)==5:
            for k in ['wall_ns','first_response_ns','peak_rss_kib']:
                values=[r[k] for r in group];entry[k]=dict(min=min(values),median=statistics.median(values),max=max(values))
        summary.append(entry)
    result=dict(planned=324,recorded=len(rows),missing=324-len(rows),failed_rows=failures,
                failed_builds=[r['label'] for r in builds if r['status']!='complete'], missing_builds=build_order[len(builds):],
                complete_primary_cells=sum(s['complete'] for s in summary),freeze_verification='archived' if archived else 'live')
    compiler_summary=[]
    groups={r['label'].rsplit('-',1)[0] for r in builds if r['label'].startswith(('generate-','build-native-','install-','prereq-native-'))}
    for group in sorted(groups):
        samples=[r for r in builds if r['label'].rsplit('-',1)[0]==group and r['status']=='complete']
        entry=dict(group=group,complete=len(samples)==3,samples=samples)
        if len(samples)==3:
            for k in ['wall_ns','peak_rss_kib']:
                values=[r[k] for r in samples];entry[k]=dict(min=min(values),median=statistics.median(values),max=max(values))
        compiler_summary.append(entry)
    write_json(out/'build-summary.json',compiler_summary)
    write_json(out/'audit.json',result);write_json(out/'summary.json',summary);return result


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan',action='store_true');parser.add_argument('--audit',type=Path)
    parser.add_argument('--archived',action='store_true');parser.add_argument('--out',type=Path)
    args=parser.parse_args()
    if args.plan: print(json.dumps(manifest(),indent=2));return
    if args.audit:
        try: result=audit(args.audit,args.archived)
        except (AssertionError,ValueError,KeyError,OSError) as error:
            result=dict(status='invalid',error=str(error) or repr(error));write_json(args.audit/'audit.json',result);print(json.dumps(result));raise SystemExit(1)
        print(json.dumps(result))
        if result['missing'] or result['failed_rows'] or result['failed_builds'] or result['missing_builds']:raise SystemExit(1)
        return
    if args.out is None:parser.error('--out required to launch')
    launch(args.out.resolve())


if __name__=='__main__':main()
