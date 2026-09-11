"""Losslessly pack verbose per-step receipts and record their byte hashes."""
import gzip,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
def main():
    for name in ['s01-probe-projection','s01-probe-projection-repair','s01-probe-strong-control']:
        out=ROOT/'docs/experiments/results'/name;manifest={}
        for p in out.glob('*.json'):
            data=p.read_bytes()
            if b'"stdout":' not in data:continue
            packed=gzip.compress(data,mtime=0);target=p.with_suffix('.json.gz');target.write_bytes(packed)
            assert gzip.decompress(target.read_bytes())==data
            manifest[p.name]=dict(raw_sha256=hashlib.sha256(data).hexdigest(),gzip_sha256=hashlib.sha256(packed).hexdigest(),raw_bytes=len(data),packed_bytes=len(packed))
            p.unlink()
        if manifest:(out/'packed-receipts.json').write_text(json.dumps(manifest,indent=2)+'\n')
if __name__=='__main__':main()
