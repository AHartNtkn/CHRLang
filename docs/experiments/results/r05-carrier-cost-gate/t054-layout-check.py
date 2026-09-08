from pathlib import Path
import re,json
result={}
for mode in ['off','on']:
 path=next(Path('/tmp/t054-layout-'+mode+'/debug/deps').glob('chr_compiled-*.ll'))
 text=path.read_text(); definitions=dict(re.findall(r'^(!\d+) = (.*)$',text,re.M))
 root=next(key for key,value in definitions.items() if value.startswith('!DINamespace(name: "chr_compiled",'))
 types={}
 for name in ['Engine','Prepared','Stats']:
  value=next(v for v in definitions.values() if v.startswith('!DICompositeType(tag: DW_TAG_structure_type, name: "'+name+'", scope: '+root+','))
  size=int(re.search(r' size: (\d+)',value)[1])//8
  elements=definitions[re.search(r' elements: (!\d+)',value)[1]]
  fields=[re.search(r'name: "([^"]+)"',definitions[e])[1] for e in re.findall(r'!\d+',elements) if 'DW_TAG_member' in definitions[e]]
  types[name]={'bytes':size,'carrier_fields':[f for f in fields if f.startswith('carrier')]}
 hooks={name: name in text for name in ['carrier_tick','carrier_entry','carrier_inserted','contract_carriers_inferred','carrier_eligibility']}
 assert all(hooks.values()) if mode=='on' else not any(hooks.values())
 assert (types['Engine']['carrier_fields']==['carrier','carrier_blocked'] or set(types['Engine']['carrier_fields'])=={'carrier','carrier_blocked'}) if mode=='on' else types['Engine']['carrier_fields']==[]
 assert types['Prepared']['carrier_fields']==(['carrier'] if mode=='on' else [])
 result[mode]={'artifact':str(path),'types':types,'hooks_present':hooks}
print(json.dumps(result,indent=2))
