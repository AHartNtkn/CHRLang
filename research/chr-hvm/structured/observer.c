// First-order incremental native collapse. Reduction and constructor traversal
// have separate service boundaries; this is not a CHR rule evaluator.
typedef struct { u64 loc; u32 next; int entered; } ChrFrame;
typedef struct ChrObservation {
  ChrFrame frames[256];
  u32 depth;
  struct ChrObservation *next;
} ChrObservation;
typedef struct { ChrObservation *head, *tail; u64 len; } ChrQueue;
fn void chr_push(ChrQueue *q, ChrObservation *s) {
  s->next = NULL;
  if (q->tail) q->tail->next = s; else q->head = s;
  q->tail = s; q->len++;
}
fn ChrObservation *chr_new(u64 loc) {
  ChrObservation *s = calloc(1, sizeof(ChrObservation));
  if (!s) abort();
  s->depth = 1; s->frames[0].loc = loc;
  return s;
}
// Return1 when this root has completed, split, failed or been rejected.
fn int chr_observe_step(ChrObservation *s, ChrQueue *q, u64 *unsupported) {
  ChrFrame *f = &s->frames[s->depth-1];
  Term term = heap_read(f->loc);
  if (!f->entered) {
    CHR_ACTIVE = 1;
    term = wnf_at(f->loc);
    CHR_ACTIVE = 0;
    if (CHR_YIELDED) return 0;
    u8 tag = term_tag(term);
    if (tag == SUP || tag == ERA) {
      if (s->depth == 1) {
        if (tag == SUP) {
          chr_push(q, chr_new(term_val(term)));
          chr_push(q, chr_new(term_val(term)+1));
        }
        return 1;
      }
      s->depth--;
      ChrFrame *parent = &s->frames[s->depth-1];
      Term enclosing = heap_read(parent->loc);
      if (tag == ERA) {
        heap_set(parent->loc, term_new_era());
      } else {
        u32 arity = term_arity(enclosing);
        u32 chosen = parent->next-1;
        u32 label = term_ext(term);
        Term left[16], right[16];
        for (u32 i=0; i<arity; i++) {
          if (i == chosen) {
            left[i] = heap_read(term_val(term));
            right[i] = heap_read(term_val(term)+1);
          } else {
            Copy c = term_clone(label, heap_read(term_val(enclosing)+i));
            left[i] = c.k0; right[i] = c.k1;
          }
        }
        Term a = term_new_at(term_val(enclosing), term_tag(enclosing), term_ext(enclosing), arity, left);
        Term b = term_new_(term_tag(enclosing), term_ext(enclosing), arity, right);
        heap_set(parent->loc, term_new_sup(label,a,b));
      }
      parent->entered = 0; parent->next = 0;
      return 0;
    }
    if (tag == NUM || (tag >= C00 && tag <= C16)) {
      f->entered = 1; f->next = 0;
      return 0;
    }
    // An unexhausted weak reduction can return a rebuilt computational term.
    if (CHR_VISITS > 0) return 0;
    (*unsupported)++;
    return 1;
  }
  u32 arity = term_arity(term);
  if (f->next < arity) {
    if (s->depth == 256) {
      fprintf(stderr,"native observer traversal depth bound\n"); exit(2);
    }
    u64 child = term_val(term)+f->next++;
    s->frames[s->depth++] = (ChrFrame){.loc=child};
    return 0;
  }
  if (s->depth == 1) {
    print_term_quoted(term); printf("\n");
    return 1;
  }
  s->depth--;
  return 0;
}
fn void chr_service(Term root) {
  u64 fuel = strtoull(getenv("CHR_FUEL"),NULL,10);
  u64 limit = strtoull(getenv("CHR_CALLS"),NULL,10);
  if (fuel==0 || fuel>8 || limit>1024) abort();
  wnf_set_itrs_enabled(getenv("CHR_COUNTERS_OFF")==NULL);
  wnf_stack_init();
  ChrQueue q = {0};
  u64 loc = heap_alloc(1); heap_set(loc,root); chr_push(&q,chr_new(loc));
  u64 calls=0, unsupported=0;
  while (calls<limit && q.head) {
    ChrObservation *s=q.head;
    q.head=s->next; if (!q.head) q.tail=NULL; q.len--;
    CHR_QUOTA=fuel; CHR_VISITS=0; CHR_YIELDED=0;
    u64 before=ITRS;
    int done=chr_observe_step(s,&q,&unsupported);
    if (done) free(s); else chr_push(&q,s);
    calls++;
    fprintf(stderr,"{\"call\":%llu,\"delta\":%llu,\"visits\":%llu,\"pending\":%llu,\"stack\":%u}\n",
      (unsigned long long)calls,(unsigned long long)(ITRS-before),(unsigned long long)CHR_VISITS,
      (unsigned long long)q.len,WNF_S_POS);
  }
  fprintf(stderr,"{\"calls\":%llu,\"pending\":%llu,\"unsupported\":%llu}\n",
    (unsigned long long)calls,(unsigned long long)q.len,(unsigned long long)unsupported);
  while (q.head) { ChrObservation *s=q.head; q.head=s->next; free(s); }
}
