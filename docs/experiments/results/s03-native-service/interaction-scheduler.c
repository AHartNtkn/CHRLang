// Separate experimental scheduler over retained native heap locations.
// Deliberately admits leaf observations only; it does not implement CNF.
fn void chr_service(Term root) {
  u64 fuel = strtoull(getenv("CHR_FUEL"), NULL, 10);
  u64 limit = strtoull(getenv("CHR_CALLS"), NULL, 10);
  if (fuel == 0 || fuel > 8 || limit > 64) abort();
  wnf_set_itrs_enabled(1);
  EvalCollapseQueue queue;
  eval_collapse_queue_init(&queue);
  u64 loc = heap_alloc(1);
  heap_set(loc, root);
  eval_collapse_queue_push(&queue, 0, 0, loc);
  u64 calls = 0;
  u64 unsupported = 0;
  EvalCollapseTask task;
  while (calls < limit && eval_collapse_queue_pop(&queue, &task)) {
    u64 before = ITRS;
    STEPS_ITRS_LIM = before + fuel;
    Term term = wnf_at(task.loc);
    STEPS_ITRS_LIM = 0;
    calls++;
    switch (term_tag(term)) {
      case SUP: {
        u64 children = term_val(term);
        eval_collapse_queue_push(&queue, task.key + 1, 0, children);
        eval_collapse_queue_push(&queue, task.key + 1, 0, children + 1);
        break;
      }
      case ERA: break;
      case C00:
      case NUM:
        print_term_quoted(term);
        printf("\n");
        break;
      default:
        if (ITRS > before) {
          eval_collapse_queue_push(&queue, task.key + 1, 0, task.loc);
        } else {
          unsupported++;
        }
    }
    fprintf(stderr, "{\"call\":%llu,\"delta\":%llu,\"pending\":%llu,\"stack\":%u}\n",
      (unsigned long long)calls, (unsigned long long)(ITRS-before),
      (unsigned long long)queue.len, WNF_S_POS);
  }
  fprintf(stderr, "{\"calls\":%llu,\"pending\":%llu,\"unsupported\":%llu}\n",
    (unsigned long long)calls, (unsigned long long)queue.len,
    (unsigned long long)unsupported);
  eval_collapse_queue_free(&queue);
}
