// Separate consumer bytes; no native heap locations escape this encoder.
typedef struct { unsigned char *data; size_t len, capacity; } WireBuffer;
static void wire_byte(WireBuffer *b, unsigned char x) {
  if (b->len == b->capacity) {
    assert(b->capacity <= SIZE_MAX / 2);
    size_t capacity = b->capacity ? b->capacity * 2 : 8;
    unsigned char *p = realloc(b->data, capacity); assert(p);
    b->data = p; b->capacity = capacity;
  }
  b->data[b->len++] = x;
}
static void wire_word(WireBuffer *b, u32 n) {
  for (unsigned i = 0; i < 4; i++) wire_byte(b, (unsigned char)(n >> (8 * i)));
}
static int wire_named(Term t, const char *name, unsigned arity) {
  return term_tag(t) >= C00 && term_tag(t) <= C16 && term_arity(t) == arity &&
    term_ext(t) < TABLE.len && strcmp(TABLE.data[term_ext(t)], name) == 0;
}
static Term wire_field(Term t, unsigned i) { return heap_read(term_val(t) + i); }
static u32 wire_number(Term t) { assert(term_tag(t) == NUM && term_val(t) <= UINT32_MAX); return (u32)term_val(t); }
static void wire_terms(WireBuffer *b, Term list) {
  while (wire_named(list, "Cons", 2)) {
    Term t = wire_field(list, 0);
    if (wire_named(t, "Var", 1)) wire_byte(b, 1);
    else { assert(wire_named(t, "Atom", 1)); wire_byte(b, 2); }
    wire_word(b, wire_number(wire_field(t, 0)));
    list = wire_field(list, 1);
  }
  assert(wire_named(list, "Nil", 0)); wire_byte(b, 0);
}
static void wire_emit(WireBuffer *b, Term root) {
  assert(wire_named(root, "Answer", 2)); wire_byte(b, 4);
  wire_terms(b, wire_field(root, 0));
  Term list = wire_field(root, 1);
  while (wire_named(list, "Cons", 2)) {
    Term fact = wire_field(list, 0); assert(wire_named(fact, "Result", 2));
    wire_byte(b, 3); wire_word(b, wire_number(wire_field(fact, 0)));
    wire_terms(b, wire_field(fact, 1)); list = wire_field(list, 1);
  }
  assert(wire_named(list, "Nil", 0)); wire_byte(b, 0);
}
