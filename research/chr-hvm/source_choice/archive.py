"""Losslessly archive verbose native event receipts with deterministic gzip headers."""
import gzip,hashlib,json,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s03-native-choice-identity'
def digest(path):
 with path.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def main():
 receipts={}
 for name in ['runs.jsonl','diagnostics.jsonl','deterministic.jsonl']:
  source=OUT/name;target=OUT/(name+'.gz');expected=digest(source);size=source.stat().st_size
  with source.open('rb') as src,target.open('wb') as dst:
   with gzip.GzipFile(filename='',mode='wb',fileobj=dst,mtime=0) as gz:shutil.copyfileobj(src,gz)
  with gzip.open(target,'rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==expected
  receipts[name]=dict(uncompressed_sha256=expected,uncompressed_bytes=size,archive_sha256=digest(target),archive_bytes=target.stat().st_size)
  source.unlink()
 (OUT/'archives.json').write_text(json.dumps(receipts,indent=2)+'\n')
 print('All three archives reproduce the exact original receipt bytes.')
if __name__=='__main__':main()
