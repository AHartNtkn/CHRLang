import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s10-native-prepared';BUILD=ROOT/'target/s10-native-prepared'
def main():
 BUILD.mkdir(exist_ok=True);source=ROOT/'target/s03-native-identity-source/native.c'
 prior=json.loads((ROOT/'docs/experiments/results/s03-native-identity-source/validation.json').read_text());assert hashlib.sha256(source.read_bytes()).hexdigest()==prior['hashes'][str(source.relative_to(ROOT))]
 (BUILD/'native.c').write_bytes(source.read_bytes());(BUILD/'harness.c').write_bytes(Path(__file__).with_name('harness.c').read_bytes())
 p=subprocess.run(['clang','-O2','-Wall',str(BUILD/'harness.c'),'-o',str(BUILD/'prepared')],capture_output=True,text=True,timeout=60)
 (OUT/'build.json').write_text(json.dumps(dict(command=p.args,code=p.returncode,stdout=p.stdout,stderr=p.stderr,source_sha256=hashlib.sha256(source.read_bytes()).hexdigest()),indent=2)+'\n');assert p.returncode==0,p.stderr
 print('Prepared harness builds against unchanged native reducer source.')
if __name__=='__main__':main()
