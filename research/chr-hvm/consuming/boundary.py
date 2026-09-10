"""Explicit native birth-namespace exhaustion must remain an incomplete outcome."""
import json,sys
sys.dont_write_bytecode=True
from gate import OUT,BINARY,native

def main():
    source=(OUT/'binary-1-0.hvm').read_text()
    lines=source.splitlines();assert lines[-1].startswith('@main = ') and lines[-1].endswith(',1)')
    lines[-1]=lines[-1][:-3]+',8388608)'
    path=OUT/'choice-namespace-bound.hvm';path.write_text('\n'.join(lines)+'\n')
    result=native.invoke(BINARY,path,8,4096)
    assert result.get('code')==0 and result['stdout'].splitlines()==['#ChoiceLimit{}'],result
    # The ordinary CHR source has two successful alternatives; this is unknown,
    # not a successful empty store, a failed branch or an exhausted CHR query.
    record=dict(result=result,classification='incomplete: native choice namespace exhausted')
    (OUT/'choice-namespace-bound.json').write_text(json.dumps(record)+'\n')
    print(record['classification'])
if __name__=='__main__':main()
