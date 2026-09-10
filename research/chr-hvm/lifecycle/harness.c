#define _GNU_SOURCE
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

static void emit_answer(uint64_t term);
#define HVM_NO_MAIN
#include "native.c"

static uint64_t now_ns(void) {
  struct timespec t;
  assert(clock_gettime(CLOCK_MONOTONIC, &t) == 0);
  return (uint64_t)t.tv_sec * 1000000000ULL + (uint64_t)t.tv_nsec;
}

typedef struct { char *text; size_t len; unsigned query; } OwnedAnswer;
typedef struct {
  uint64_t setup, observer_setup, service, serialization, pending_drop, export, drop;
  uint64_t first, consumer_drop;
  int observed;
} QueryTimes;
static QueryTimes *active_times;
static uint64_t service_start;
static FILE *answer_stream;

static void emit_answer(uint64_t term) {
  uint64_t start = now_ns();
  print_term_quoted(term);
  printf("\n");
  // Publish complete bytes to the stream owner before claiming first observation.
  assert(fflush(answer_stream) == 0);
  uint64_t end = now_ns();
  active_times->serialization += end - start;
  if (!active_times->observed) {
    active_times->observed = 1;
    active_times->first = end - service_start;
  }
}

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
static void session_free(void){
 for(u32 i=0;i<TABLE.len;i++){free(TABLE.data[i]);TABLE.data[i]=NULL;}TABLE.len=0;
 for(u32 i=0;i<PARSE_SEEN_FILES_LEN;i++){free(PARSE_SEEN_FILES[i]);PARSE_SEEN_FILES[i]=NULL;}PARSE_SEEN_FILES_LEN=0;PARSE_BINDS_LEN=0;PARSE_FRESH_LAB=0x800000u;PARSE_FORK_SIDE=-1;
 runtime_free();BOOK=NULL;TABLE.data=NULL;HEAP_NEXT=1;
}

static void publish(const OwnedAnswer *a) {
  printf("RESULT %u %zu\n", a->query, a->len);
  assert(fwrite(a->text, 1, a->len, stdout) == a->len);
}

static uint64_t query_total(const QueryTimes *t) {
  return t->setup + t->observer_setup + t->service + t->pending_drop + t->export + t->drop;
}

static int session(const char *path) {
  uint64_t start = now_ns();
  runtime_init(0, 0, 0);
  uint64_t runtime_init_ns = now_ns() - start;
  start = now_ns();
  char *src = sys_file_read(path); assert(src);
  uint64_t source_load_ns = now_ns() - start;
  start = now_ns();
  u32 main_id, entry;
  assert(runtime_prepare(&main_id, path, src));
  assert(runtime_entry("s_r0", &entry));
  const char *names[] = {"Nil", "Cons", "Var", "Atom", "Fact", "State"};
  for (unsigned i = 0; i < 6; i++) table_find(names[i], strlen(names[i]));
  u64 seal = HEAP_NEXT;
  uint64_t prepare_ns = now_ns() - start;
  start = now_ns(); free(src);
  uint64_t source_drop_ns = now_ns() - start;
  start = now_ns();
  unsigned count; assert(scanf("%u", &count) == 1 && count <= 4096);
  OwnedAnswer *held = calloc(count, sizeof(OwnedAnswer)); assert(held);
  uint64_t consumer_setup_ns = now_ns() - start;
  uint64_t query_sum = 0; u64 peak_words = 0;
  for (unsigned i = 0; i < count; i++) {
    QueryTimes times = {0}; active_times = &times;
    start = now_ns();
    unsigned limit, keep;
    Term state = query_state(&limit, &keep);
    Term root = term_new_app(term_new_ref(entry), state);
    times.setup = now_ns() - start;
    start = now_ns();
    FILE *saved = stdout; char *bytes = NULL; size_t len = 0;
    answer_stream = open_memstream(&bytes, &len); assert(answer_stream);
    stdout = answer_stream;
    wnf_set_itrs_enabled(0); wnf_stack_init();
    ChrQueue queue = {0}; u64 loc = heap_alloc(1); heap_set(loc, root);
    chr_push(&queue, chr_new(loc));
    times.observer_setup = now_ns() - start;
    u64 calls = 0, unsupported = 0;
    service_start = now_ns();
    while (calls < limit && queue.head) {
      ChrObservation *s = queue.head; queue.head = s->next;
      if (!queue.head) queue.tail = NULL;
      queue.len--;
      CHR_QUOTA = 8; CHR_VISITS = 0; CHR_YIELDED = 0;
      int done = chr_observe_step(s, &queue, &unsupported);
      if (done) free(s); else chr_push(&queue, s);
      calls++;
    }
    times.service = now_ns() - service_start;
    u64 pending = queue.len;
    start = now_ns();
    while (queue.head) { ChrObservation *s = queue.head; queue.head = s->next; free(s); }
    times.pending_drop = now_ns() - start;
    start = now_ns();
    assert(fclose(answer_stream) == 0); stdout = saved; answer_stream = NULL;
    OwnedAnswer answer = {.text = malloc(len + 1), .len = len, .query = i};
    assert(answer.text); memcpy(answer.text, bytes, len + 1); free(bytes);
    times.export = now_ns() - start;
    u64 used = HEAP_NEXT - seal; if (used > peak_words) peak_words = used;
    start = now_ns(); wnf_stack_free(); HEAP_NEXT = seal;
    times.drop = now_ns() - start;
    if (keep) held[i] = answer;
    else {
      publish(&answer);
      start = now_ns(); free(answer.text); times.consumer_drop = now_ns() - start;
    }
    query_sum += query_total(&times) + times.consumer_drop;
    assert(times.serialization <= times.service);
    fprintf(stderr, "{\"query\":%u,\"calls\":%llu,\"pending\":%llu,\"unsupported\":%llu,"
      "\"dynamic_words\":%llu,\"retained_bytes\":%zu,\"allocator\":\"ordinary\","
      "\"query_setup_ns\":%llu,\"observer_setup_ns\":%llu,\"service_ns\":%llu,"
      "\"serialization_ns\":%llu,\"compute_traverse_ns\":%llu,\"pending_drop_ns\":%llu,"
      "\"export_ns\":%llu,\"query_drop_ns\":%llu,\"consumer_drop_ns\":%llu,\"query_total_ns\":%llu,\"first_observation_ns\":",
      i,(unsigned long long)calls,(unsigned long long)pending,(unsigned long long)unsupported,
      (unsigned long long)used,len+1,(unsigned long long)times.setup,(unsigned long long)times.observer_setup,
      (unsigned long long)times.service,(unsigned long long)times.serialization,
      (unsigned long long)(times.service-times.serialization),(unsigned long long)times.pending_drop,
      (unsigned long long)times.export,(unsigned long long)times.drop,
      (unsigned long long)times.consumer_drop,(unsigned long long)query_total(&times));
    if (times.observed) fprintf(stderr, "%llu", (unsigned long long)times.first);
    else fprintf(stderr, "null");
    fprintf(stderr, "}\n");
  }
  start = now_ns(); session_free();
  uint64_t prepared_drop_ns = now_ns() - start;
  uint64_t consumer_drop_ns = 0;
  for (unsigned i = 0; i < count; i++) if (held[i].text) {
    publish(&held[i]); start = now_ns(); free(held[i].text);
    consumer_drop_ns += now_ns() - start;
  }
  start = now_ns(); free(held); consumer_drop_ns += now_ns() - start;
  uint64_t lifecycle = runtime_init_ns + source_load_ns + prepare_ns + source_drop_ns +
    consumer_setup_ns + query_sum + prepared_drop_ns + consumer_drop_ns;
  fprintf(stderr, "{\"session_disposed\":true,\"allocator\":\"ordinary\","
    "\"prepared_words\":%llu,\"peak_dynamic_words\":%llu,"
    "\"runtime_init_ns\":%llu,\"source_load_ns\":%llu,\"prepare_ns\":%llu,"
    "\"source_drop_ns\":%llu,\"consumer_setup_ns\":%llu,\"prepared_drop_ns\":%llu,"
    "\"consumer_drop_ns\":%llu,\"lifecycle_ns\":%llu}\n",
    (unsigned long long)seal,(unsigned long long)peak_words,(unsigned long long)runtime_init_ns,
    (unsigned long long)source_load_ns,(unsigned long long)prepare_ns,(unsigned long long)source_drop_ns,
    (unsigned long long)consumer_setup_ns,(unsigned long long)prepared_drop_ns,
    (unsigned long long)consumer_drop_ns,(unsigned long long)lifecycle);
  return 0;
}
int main(int argc, char **argv) {
  if (argc >= 3 && strcmp(argv[1], "--sessions") == 0) {
    for (int i = 2; i < argc; i++) { printf("SESSION %d\n", i - 2); session(argv[i]); }
    return 0;
  }
  assert(argc == 2); return session(argv[1]);
}
