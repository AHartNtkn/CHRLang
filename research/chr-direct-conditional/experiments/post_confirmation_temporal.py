"""Post hoc descriptive stability check; no change to registered inference."""
import json, math, statistics
from pathlib import Path
BASE = Path(__file__).resolve().parents[3] / 'docs/experiments/results/s03-post-confirmation'
def main():
    audit = json.loads((BASE / 'audit.json').read_text()); rows = []
    for contrast in audit['contrasts']:
        ratios = contrast['ratios']; logs = list(map(math.log, ratios))
        early = statistics.median(ratios[:32]); late = statistics.median(ratios[32:])
        label = contrast['classification']
        crossing = (label == 'faster' and max(early, late) >= .9) or (label == 'slower' and min(early, late) <= 1.1)
        rows.append({k: contrast[k] for k in ['scenario', 'demand', 'control', 'classification']} | dict(first_half_median=early, second_half_median=late, lag_one_log_correlation=statistics.correlation(logs[:-1], logs[1:]), classification_half_threshold_crossing=crossing))
    print(json.dumps(dict(purpose='Post hoc descriptive temporal check; does not revise registered classifications or add confirmatory inference.', chronological_groups='First and last 32 matched blocks for each contrast; all samples retained.', confirmed_direction_half_threshold_crossings=sum(r['classification_half_threshold_crossing'] for r in rows), absolute_lag_correlation_over_half=sum(abs(r['lag_one_log_correlation']) > .5 for r in rows), max_absolute_lag_correlation=max(abs(r['lag_one_log_correlation']) for r in rows), rows=rows), indent=2))
if __name__ == '__main__': main()
