import json
import os
import sys
from time import perf_counter_ns as clock

def pair():
    t=clock()
    return clock()-t

def bulk(clocked):
    values=[0]*100000
    start=clock()
    for i in range(100000):
        values[i]=pair() if clocked else i
    elapsed=clock()-start
    if not clocked:
        assert all(i==value for i,value in enumerate(values))
    return elapsed

for _ in range(1000):pair()
samples=[pair() for _ in range(10000)]
if int(sys.argv[1])==0:
    baseline=bulk(False);measured=bulk(True)
else:
    measured=bulk(True);baseline=bulk(False)
print(json.dumps(dict(cpu=next(iter(os.sched_getaffinity(0))),baseline_ns=baseline,pairs_ns=measured,baseline_verified=True,samples=samples)))
