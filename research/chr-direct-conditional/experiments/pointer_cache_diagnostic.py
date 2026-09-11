from pathlib import Path
import shutil, zipfile
root=Path(__file__).resolve().parents[3];p=Path('/tmp/chr-static-pointer-probe');(p/'src/demand').mkdir(parents=True,exist_ok=True)
(p/'Cargo.toml').write_text(f'''[package]\nname="chr-pointer-probe"\nversion="0.1.0"\nedition="2024"\n[features]\nwork-diagnostics=[]\ncandidate-profile=[]\n[dependencies]\nchr-syntax={{path="{root}/crates/chr-syntax"}}\nchr-observe={{path="{root}/research/chr-observe",default-features=false}}\n''')
archive=zipfile.ZipFile(root/'docs/experiments/results/s10-static-control/sources.zip');s=archive.read('research/chr-direct-choice/src/demand.rs').decode();s=s.replace('            ground.insert(key, id);','''            let before_len=ground.len();let start=crate::meter::begin();
            ground.insert(key,id);
            crate::record(key as usize,before_len,crate::meter::end(start));''');(p/'src/demand.rs').write_text(s)
(p/'src/demand/templates.rs').write_bytes(archive.read('research/chr-direct-choice/src/demand/templates.rs'))
for name in ['post_continuation_source','static_posts','value_choices']:(p/f'src/{name}.rs').write_bytes(archive.read(f'research/chr-direct-conditional/examples/support/{name}.rs'))
(p/'src/meter.rs').write_bytes(archive.read('research/chr-compiled/experiments/meter.rs'))
(p/'src/main.rs').write_text('''#![allow(dead_code)]
mod demand;mod post_continuation_source;mod static_posts;mod value_choices;mod meter;
use std::cell::RefCell;use std::collections::BTreeMap;
thread_local!{static RECORDS:RefCell<Vec<(usize,usize,usize,usize)>>=RefCell::new(Vec::with_capacity(100000));}
fn record(key:usize,len:usize,m:meter::Reading){RECORDS.with(|v|v.borrow_mut().push((key,len,m.allocation_calls,m.requested_bytes)));}
fn main(){RECORDS.with(|_|());let rules=post_continuation_source::rules("independent",3,true);let init=static_posts::Prepared::infer(&rules).unwrap();let rules=value_choices::lower(init.rules());let p=demand::Prepared::new(rules).unwrap().with_miss_reuse().with_derivation_templates();
for seed in 0..4 {let q=post_continuation_source::query("independent",3,0,seed%2,false);let mut run=p.start(init.initialize(&q)).unwrap();let mut expected=post_continuation_source::expected("independent",3,seed%2,true);let mut done=false;
for _ in 0..2000000{match run.tick(){demand::Event::Answer(a)=>{let i=expected.iter().position(|e|chr_observe::equivalent(e,&a,&mut Default::default())).unwrap();expected.swap_remove(i);},demand::Event::Exhausted=>{done=true;break},_=>()}}assert!(done&&expected.is_empty());}
let rows=RECORDS.with(|r|std::mem::take(&mut *r.borrow_mut()));let mut groups:Vec<Vec<(usize,usize,usize,usize)>>=vec![];for r in rows{if r.1==0{groups.push(vec![]);}groups.last_mut().unwrap().push(r);}
let mut observed=0;let mut replayed=0;let mut ordered=0;let mut differing=0;
for g in &groups {let actual:usize=g.iter().map(|r|r.3).sum();let mut map=BTreeMap::new();let start=meter::begin();for r in g {map.insert(r.0,0usize);}let bytes=meter::end(start).requested_bytes;assert_eq!(actual,bytes);drop(map);
let mut map=BTreeMap::new();let start=meter::begin();for i in 0..g.len(){map.insert(i,0usize);}let stable=meter::end(start).requested_bytes;drop(map);observed+=actual;replayed+=bytes;ordered+=stable;if actual!=stable{differing+=1;println!("GROUP nodes={} observed={} replay={} ordered={} delta={}",g.len(),actual,bytes,stable,actual as isize-stable as isize);}}
println!("TOTAL groups={} observed={} replay={} ordered={} differing={}",groups.len(),observed,replayed,ordered,differing);
}
''')
