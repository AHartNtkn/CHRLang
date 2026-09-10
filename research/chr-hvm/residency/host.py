"""Diagnostic-only host phase tracing; the primary host source remains authoritative."""
import json,sys,tracemalloc
from pathlib import Path
root=Path(__file__).resolve().parents[3]
records=[]
def rss():
    with open('/proc/self/smaps_rollup') as stream:
        for line in stream:
            if line.startswith('Rss:'):return int(line.split()[1])
    raise RuntimeError('RSS unavailable')
class TracePhases(dict):
    def __setitem__(self,name,value):
        current,peak=tracemalloc.get_traced_memory()
        metadata=tracemalloc.get_tracemalloc_memory()
        records.append(dict(phase=name,traced_current=current,traced_peak=peak,tracer_metadata=metadata,host_rss_kib=rss()))
        super().__setitem__(name,value)
        tracemalloc.reset_peak()
path=root/'research/chr-hvm/host_session/run.py'
source=path.read_text();assert source.count('phases={}\n')==1
source=source.replace('phases={}\n','phases=TracePhases()\n')
namespace=dict(__name__='diagnostic_host',__file__=str(path),TracePhases=TracePhases)
exec(compile(source,str(path),'exec'),namespace)
tracemalloc.start(10)
namespace['main']()
current,peak=tracemalloc.get_traced_memory()
returned_rss=rss()
snapshot=tracemalloc.take_snapshot()
retained=[dict(bytes=s.size,count=s.count,traceback=[str(f) for f in s.traceback]) for s in snapshot.statistics('traceback')[:20]]
tracemalloc.stop()
print(json.dumps(dict(host_memory=True,scope='post-import Python tracing including checkpoint bookkeeping; RSS includes tracer',phases=records,returned_current=current,returned_peak=peak,returned_host_rss_kib=returned_rss,retained=retained)),file=sys.stderr)
