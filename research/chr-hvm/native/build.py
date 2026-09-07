"""Build pinned baseline and separately instrumented trace executables."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

PIN = '6defdfc7dae2a3cca5dd6e74ed0612385b5646a8'


def build(checkout, destination):
    revision = subprocess.check_output(['git', '-C', str(checkout), 'rev-parse', 'HEAD'], text=True).strip()
    assert revision == PIN
    subprocess.run(['git', '-C', str(checkout), 'diff', '--exit-code', 'HEAD', '--', 'src/hvm.c'], check=True)
    source = (checkout / 'src/hvm.c').read_text()
    instrumented = source
    hooks = {
        'fn Term wnf_dup_lam(u32 lab, u64 loc, u8 side, Term lam) {':
            'fprintf(stderr,"{\\"event\\":\\"dup_lam\\",\\"label\\":%u,\\"introduces_sup\\":%s}\\n",lab,(term_ext(lam)&LAM_ERA_MASK)?"false":"true");',
        'fn Term wnf_dup_sup(u32 lab, u64 loc, u8 side, Term sup) {':
            'fprintf(stderr,"{\\"event\\":\\"dup_sup\\",\\"dup_label\\":%u,\\"sup_label\\":%u}\\n",lab,term_ext(sup));',
        'fn Term wnf_dsu_num(Term lab_num, Term a, Term b) {':
            'fprintf(stderr,"{\\"event\\":\\"dsu_num\\",\\"requested\\":%u,\\"effective\\":%u}\\n",(u32)term_val(lab_num),(u32)term_val(lab_num)&EXT_MASK);',
        'fn Term wnf_ddu_num(Term lab_num, Term val, Term bod) {':
            'fprintf(stderr,"{\\"event\\":\\"ddu_num\\",\\"requested\\":%u,\\"effective\\":%u}\\n",(u32)term_val(lab_num),(u32)term_val(lab_num)&EXT_MASK);',
    }
    for signature, event in hooks.items():
        assert instrumented.count(signature) == 1
        instrumented = instrumented.replace(signature, signature + '\n  ' + event)
    destination.mkdir(parents=True, exist_ok=True)
    trace_source = destination / 'hvm-trace.c'
    trace_source.write_text(instrumented)
    commands = []
    for name, path in [('baseline', checkout / 'src/hvm.c'), ('trace', trace_source)]:
        command = ['clang', '-O2', '-o', str(destination / name), str(path)]
        subprocess.run(command, check=True)
        commands.append(command)
    manifest = dict(revision=revision, compiler=subprocess.check_output(['clang', '--version'], text=True),
                    source_sha256=hashlib.sha256(source.encode()).hexdigest(),
                    instrumented_sha256=hashlib.sha256(instrumented.encode()).hexdigest(),
                    commands=commands, hooks=hooks)
    (destination / 'build.json').write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('checkout', type=Path)
    p.add_argument('destination', type=Path)
    a = p.parse_args()
    build(a.checkout, a.destination)
