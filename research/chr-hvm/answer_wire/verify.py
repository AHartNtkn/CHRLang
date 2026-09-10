"""Sanitized native replays and ordinary-output regression after wire integration."""
import json,subprocess
from gate import ROOT,OUT,BUILD,invoke,common,native_check,native_bounds

def main():
    command=['clang','-O1','-fsanitize=undefined','-fno-sanitize-recover=all',str(BUILD/'ownership/harness.c'),'-o',str(BUILD/'ownership/ubsan')]
    p=subprocess.run(command,capture_output=True,text=True,timeout=60)
    (OUT/'sanitized-build.json').write_text(json.dumps(dict(command=command,code=p.returncode,stdout=p.stdout,stderr=p.stderr),indent=2)+'\n');assert p.returncode==0,p.stderr
    plans=json.loads((ROOT/'docs/experiments/results/s10-native-prepared/plans.json').read_text())
    old=[json.loads(l) for l in (ROOT/'docs/experiments/results/s10-native-prepared/runs.jsonl').read_text().splitlines()]
    with (OUT/'sanitized.jsonl').open('w') as log:
        for plan,prior in zip(plans,old):
            row=invoke([BUILD/'ownership/ubsan',ROOT/plan['program']],plan['input'],native_bounds)
            log.write(json.dumps(dict(group=plan['group'],result=row))+'\n');log.flush();native_check(plan,row,prior)
    oldroot=ROOT/'docs/experiments/results/s10-finite-kept-read'
    rows=[json.loads(l) for l in (oldroot/'common-replays.jsonl').read_text().splitlines()]
    groups=json.loads((ROOT/'docs/experiments/results/s10-native-common-source/groups.json').read_text())
    with (OUT/'ordinary-output-replays.jsonl').open('w') as log:
        for row in rows:
            g=groups[row['group']];text=g[0]['input'].rstrip('\n')
            for s in g[1:]:text+='\nNEXT\n'+'\n'.join(s['input'].splitlines()[:2])
            result=invoke([common.BINARY,common.MODES.index(row['mode'])],text+'\n',common.bounds)
            log.write(json.dumps(dict(group=row['group'],mode=row['mode'],result=result))+'\n');log.flush()
            assert result['code']==0 and bytes.fromhex(result['stdout_hex']).decode()==row['result']['stdout'] and result['stderr']==row['result']['stderr']
    print('383 sanitized native queries and 264 ordinary-output process replays pass.')
if __name__=='__main__':main()
