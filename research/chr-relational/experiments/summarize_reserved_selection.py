"""Exploratory counts and optimistic allocation bounds from audited receipts."""
import hashlib,json,subprocess
from pathlib import Path
BASE=Path(__file__).resolve().parents[3]/'docs/experiments/results/s02-reserved-selection'
def main():
    x=json.loads((BASE/'audit.json').read_text());ss={(r['kind'],tuple(r['cell'])):r for r in x['summary']}
    root=BASE.parents[3]
    parent=json.loads(subprocess.check_output(['python3',str(root/'research/chr-relational/experiments/audit_selection_ownership.py')],text=True))
    for old in parent['summary']:
        cell=(old['family'],old['size'],old['mode'],old['retention'],old['stop']);new=ss[('meter',cell)]
        assert old['requested_bytes']==new['requested_bytes'] and old['peak_excess']==new['peak_excess'],cell
        assert len(old['phases'])==len(new['phases'])
        for a,b in zip(old['phases'],new['phases']):
            assert a==dict(phase=b['phase'],query=b['query'],**b['reading']['heap']),cell
    parent_path=root/'docs/experiments/results/s02-selection-ownership/audit.json'
    (BASE/'control-bridge.json').write_text(json.dumps(dict(configurations=len(parent['summary']),all_original_phase_allocations_and_peaks_identical=True,parent=str(parent_path.relative_to(root)),parent_sha256=hashlib.sha256(parent_path.read_bytes()).hexdigest()),indent=2))
    rows=[]
    for r in x['summary']:
        if r['kind']!='meter' or not r['cell'][2].startswith('reserved-') or r['cell'][-1]!='complete':continue
        family,n,mode,retention,stop=r['cell']
        old=ss[('meter',(family,n,'chr-'+mode.removeprefix('reserved-'),retention,stop))]
        candidates=[ss[('meter',(family,n,m,retention,stop))] for m in ['local','local-filtered','scan','indexed','sealed']]
        best=min(candidates,key=lambda a:a['requested_bytes'])
        execute=sum(p['reading']['heap']['requested_bytes'] for p in r['phases'] if p['phase']=='execute')
        rows.append(dict(cell=r['cell'],requested_bytes=r['requested_bytes'],original_bytes=old['requested_bytes'],peak_excess=r['peak_excess'],original_peak=old['peak_excess'],zero_execution_bytes=r['requested_bytes']-execute,best_allocation_control=best['cell'][2],best_control_bytes=best['requested_bytes'],optimistic_ratio=(r['requested_bytes']-execute)/best['requested_bytes']))
    paired=[r for r in x['comparisons'] if r['control'].startswith('chr-')]
    result=dict(note='Unweighted exploratory comparisons; zero-execution allocation is arithmetic, not a replacement implementation or runtime bound.',
        allocation_lower=sum(r['requested_bytes']<r['original_bytes'] for r in rows),allocation_higher=sum(r['requested_bytes']>r['original_bytes'] for r in rows),peak_lower=sum(r['peak_excess']<r['original_peak'] for r in rows),peak_higher=sum(r['peak_excess']>r['original_peak'] for r in rows),
        paired_count=len(paired),signal_sensitive=sum(r['sensitive'] for r in paired),below_09=sum(r['median_paired_ratio']<.9 for r in paired),above_11=sum(r['median_paired_ratio']>1.1 for r in paired),minimum_zero_execution_ratio=min(r['optimistic_ratio'] for r in rows),maximum_zero_execution_ratio=max(r['optimistic_ratio'] for r in rows),rows=rows)
    (BASE/'bounds.json').write_text(json.dumps(result,indent=2))
    print(json.dumps({k:v for k,v in result.items() if k!='rows'},indent=2))
if __name__=='__main__':main()
