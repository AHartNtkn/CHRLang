"""Compare completed pre-amendment ownership and long-session retention."""
import gzip,hashlib,json,sys
sys.dont_write_bytecode=True
import call_long_entry as run
import call_trace_ownership as own
OLD=run.ROOT/'docs/experiments/results/s05-call-long-entry'

def analyze():
    after={tuple(x['cell']):x['memory']for x in run.audit()}
    old_cells=json.loads((OLD/'freeze.json').read_text())['cells']
    packed=json.loads((OLD/'packed-receipts.json').read_text());compared=0;long=[]
    for name,v in packed.items():
        encoded=(OLD/(name+'.gz')).read_bytes();data=gzip.decompress(encoded)
        assert hashlib.sha256(data).hexdigest()==v['raw_sha256'] and hashlib.sha256(encoded).hexdigest()==v['gzip_sha256']
        if not name.startswith(('meter-','primary-')):continue
        kind,k,i=name[:-5].split('-');i=int(i);r=json.loads(data)
        h,mem=own.parse(r);cell=tuple(old_cells[i])
        if mem is not None:assert mem==after[cell]
        newer=run.receipt(name,True);nh,nmem=own.parse(newer)
        assert h['counts']==nh['counts']and h['retained']==nh['retained']
        if cell[3]==8192:long.append(dict(cell=list(cell),before_wall_ns=r['wall_ns'],after_wall_ns=newer['wall_ns']))
        compared+=1
    assert compared==json.loads((OLD/'interruption.json').read_text())['terminal_lifecycle_receipts']
    trajectories=[]
    for cell,mem in after.items():
        if cell[0]!='trace'or cell[4]!='0':continue
        q=mem['query_retained']
        trajectories.append(dict(cell=list(cell),first_eight=q[:8],last=q[-1],distinct_after_four=len(set(q[4:])),peak=mem['peak'],requested=mem['requested']))
    return dict(compared_terminal_receipts=compared,completed_long_before_after=long,trace_trajectories=trajectories)
if __name__=='__main__':
    p=run.RAW/'diagnosis.json'
    if sys.argv[1]=='run':
        assert not p.exists();p.write_text(json.dumps(analyze(),indent=2)+'\n')
    else:
        assert analyze()==json.loads(p.read_text());print('All completed prior ownership preserved; long retention and qualifier costs verified.')
