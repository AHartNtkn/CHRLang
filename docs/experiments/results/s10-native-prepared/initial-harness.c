#define _GNU_SOURCE
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
typedef union { max_align_t align; size_t size; } Header;
static size_t owned_live=0;
static void *owned_malloc(size_t n) { assert(n<=SIZE_MAX-sizeof(Header)); Header *h=malloc(sizeof(Header)+n); assert(h);h->size=n;owned_live+=n;return h+1; }
static void owned_free(void *p) { if(p){ Header *h=(Header*)p-1;assert(owned_live>=h->size);owned_live-=h->size;free(h); } }
static void *owned_calloc(size_t n,size_t k) { assert(!k||n<=SIZE_MAX/k);void *p=owned_malloc(n*k);memset(p,0,n*k);return p; }
static void *owned_realloc(void *p,size_t n) { if(!p)return owned_malloc(n);Header *h=(Header*)p-1;size_t old=h->size;void *q=owned_malloc(n);memcpy(q,p,old<n?old:n);owned_free(p);return q; }
static char *owned_strdup(const char *p) { size_t n=strlen(p)+1;char *q=owned_malloc(n);memcpy(q,p,n);return q; }
static void stream_bytes_free(void *p) { free(p); }
#define malloc owned_malloc
#define calloc owned_calloc
#define realloc owned_realloc
#define free owned_free
#define strdup owned_strdup
#define HVM_NO_MAIN
#include "native.c"

typedef struct { char *text; size_t len; u64 hash; unsigned query; } OwnedAnswer;
static u64 bytes_hash(const void *p,size_t n) { const unsigned char *b=p;u64 h=1469598103934665603ULL;for(size_t i=0;i<n;i++){h^=b[i];h*=1099511628211ULL;}return h; }
static Term ctr(const char *name,u32 arity,Term *args) { return term_new_ctr(table_find(name,strlen(name)),arity,args); }
static Term nil(void) { return ctr("Nil",0,NULL); }
static Term cons(Term x,Term xs) { Term args[]={x,xs};return ctr("Cons",2,args); }
static Term value(void) { char kind;unsigned n;assert(scanf(" %c %u",&kind,&n)==2);assert(kind=='V'||kind=='A');Term arg=term_new_num(n);return ctr(kind=='V'?"Var":"Atom",1,&arg); }
static Term query_state(unsigned *limit,unsigned *keep) {
 unsigned outputs,facts,fresh;assert(scanf("%u %u %u %u %u",limit,keep,&outputs,&facts,&fresh)==5);
 assert(*limit<=65536&&*keep<=1&&outputs<=4096&&facts<=4096&&fresh<1000000);
 Term *os=malloc(outputs*sizeof(Term));
 for(unsigned i=0;i<outputs;i++){unsigned n;assert(scanf("%u",&n)==1);Term arg=term_new_num(n);os[i]=ctr("Var",1,&arg);}
 Term out=nil();for(unsigned i=outputs;i>0;i--)out=cons(os[i-1],out);free(os);
 Term *fs=malloc(facts*sizeof(Term));
 for(unsigned i=0;i<facts;i++){
  unsigned pred,arity;assert(scanf("%u %u",&pred,&arity)==2&&arity<=4096);Term *as=malloc(arity*sizeof(Term));
  for(unsigned j=0;j<arity;j++)as[j]=value();
  Term args=nil();for(unsigned j=arity;j>0;j--)args=cons(as[j-1],args);free(as);
  Term fields[]={term_new_num(i),term_new_num(pred),args};fs[i]=ctr("Fact",3,fields);
 }
 Term store=nil();for(unsigned i=facts;i>0;i--)store=cons(fs[i-1],store);free(fs);
 Term fields[]={store,nil(),nil(),term_new_num(facts),term_new_num(fresh),out,term_new_num(1)};
 return ctr("State",7,fields);
}
static OwnedAnswer execute(Term root,unsigned limit,unsigned query,u64 *pending,u64 *calls,u64 *unsupported) {
 FILE *saved=stdout;char *bytes=NULL;size_t len=0;FILE *stream=open_memstream(&bytes,&len);assert(stream);stdout=stream;
 wnf_set_itrs_enabled(0);wnf_stack_init();ChrQueue q={0};u64 loc=heap_alloc(1);heap_set(loc,root);chr_push(&q,chr_new(loc));
 *calls=0;*unsupported=0;
 while(*calls<limit&&q.head){
  ChrObservation *s=q.head;q.head=s->next;if(!q.head)q.tail=NULL;q.len--;
  CHR_QUOTA=8;CHR_VISITS=0;CHR_YIELDED=0;
  int done=chr_observe_step(s,&q,unsupported);assert(CHR_VISITS<=8&&WNF_S_POS==1);
  if(done)free(s);else chr_push(&q,s);(*calls)++;
 }
 *pending=q.len;
 while(q.head){ChrObservation *s=q.head;q.head=s->next;free(s);}q.tail=NULL;q.len=0;
 assert(fclose(stream)==0);stdout=saved;
 OwnedAnswer answer={.text=malloc(len+1),.len=len,.query=query};memcpy(answer.text,bytes,len+1);stream_bytes_free(bytes);answer.hash=bytes_hash(answer.text,len);
 wnf_stack_free();assert(WNF_BANK.stack==NULL&&WNF_S_POS==0&&CHR_ACTIVE==0);
 return answer;
}
static void publish(OwnedAnswer *a){assert(a->hash==bytes_hash(a->text,a->len));printf("RESULT %u %zu\n",a->query,a->len);assert(fwrite(a->text,1,a->len,stdout)==a->len);free(a->text);a->text=NULL;}
static void session_free(void){
 for(u32 i=0;i<TABLE.len;i++){free(TABLE.data[i]);TABLE.data[i]=NULL;}TABLE.len=0;
 for(u32 i=0;i<PARSE_SEEN_FILES_LEN;i++){free(PARSE_SEEN_FILES[i]);PARSE_SEEN_FILES[i]=NULL;}PARSE_SEEN_FILES_LEN=0;PARSE_BINDS_LEN=0;PARSE_FRESH_LAB=0x800000u;PARSE_FORK_SIDE=-1;
 runtime_free();BOOK=NULL;TABLE.data=NULL;HEAP_NEXT=1;
}
int main(int argc,char **argv){
 assert(argc==2||argc==3);runtime_init(0,0,0);char *src=sys_file_read(argv[1]);assert(src);u32 main_id;assert(runtime_prepare(&main_id,argv[1],src));free(src);
 if(argc==3){size_t before=owned_live;runtime_free();printf("{\"before\":%zu,\"remaining\":%zu}\n",before,owned_live);assert(owned_live>0);return 0;}
 u32 entry;assert(runtime_entry("s_r0",&entry));
 // Intern all query constructor names before sealing prepared ownership.
 const char *names[]={"Nil","Cons","Var","Atom","Fact","State"};for(unsigned i=0;i<6;i++)table_find(names[i],strlen(names[i]));
 u64 seal=HEAP_NEXT;u32 symbols=TABLE.len;Term *snapshot=malloc(seal*sizeof(Term));memcpy(snapshot,HEAP,seal*sizeof(Term));
 u64 *book=malloc(symbols*sizeof(u64));memcpy(book,BOOK,symbols*sizeof(u64));u64 namehash=0;for(u32 i=0;i<symbols;i++)namehash^=bytes_hash(TABLE.data[i],strlen(TABLE.data[i]));
 unsigned count;assert(scanf("%u",&count)==1&&count<=4096);OwnedAnswer *held=calloc(count,sizeof(OwnedAnswer));
 size_t prepared_live=owned_live;u64 peak_words=0;
 for(unsigned i=0;i<count;i++){
  size_t before=owned_live;unsigned limit,keep;Term state=query_state(&limit,&keep);u64 pending,calls,unsupported;
  OwnedAnswer answer=execute(term_new_app(term_new_ref(entry),state),limit,i,&pending,&calls,&unsupported);
  u64 used=HEAP_NEXT-seal;if(used>peak_words)peak_words=used;
  assert(TABLE.len==symbols&&memcmp(snapshot,HEAP,seal*sizeof(Term))==0&&memcmp(book,BOOK,symbols*sizeof(u64))==0);
  u64 nh=0;for(u32 j=0;j<symbols;j++)nh^=bytes_hash(TABLE.data[j],strlen(TABLE.data[j]));assert(nh==namehash);
  assert(owned_live==before+answer.len+1);
  memset(HEAP+seal,0xDD,used*sizeof(Term));HEAP_NEXT=seal;
  fprintf(stderr,"{\"query\":%u,\"calls\":%llu,\"pending\":%llu,\"unsupported\":%llu,\"dynamic_words\":%llu,\"retained_bytes\":%zu,\"prepared_unchanged\":true,\"query_restored\":true}\n",i,(unsigned long long)calls,(unsigned long long)pending,(unsigned long long)unsupported,(unsigned long long)used,answer.len+1);
  if(keep)held[i]=answer;else publish(&answer);
 }
 for(unsigned i=0;i<count;i++)if(held[i].text)publish(&held[i]);
 assert(owned_live==prepared_live);free(held);free(book);free(snapshot);session_free();assert(owned_live==0&&HEAP==NULL&&WNF_BANK.stack==NULL&&TABLE.len==0&&PARSE_SEEN_FILES_LEN==0);
 fprintf(stderr,"{\"session_disposed\":true,\"tracked_live\":%zu,\"prepared_words\":%llu,\"peak_dynamic_words\":%llu}\n",owned_live,(unsigned long long)seal,(unsigned long long)peak_words);
 return 0;
}
