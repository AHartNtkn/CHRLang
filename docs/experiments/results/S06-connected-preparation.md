# Preparation attribution selects direct ground evaluation

**Unequal-name stars spend most preparation time constructing local relations;
six-variable equality cliques spend most of it eliminating variables.** These
are different costs. The first now uses direct ground predicate evaluation instead
of compiling a general existential theory for each ground pair.

The [registered profile](../registrations/S06-connected-preparation.md) uses four
exclusive scopes in the existing term adapter. Ordinary builds contain none of
these clocks. Twelve source configurations run in ten blocks, each with an
instrumented and ordinary process: 240 processes before the repair and 240 after.
Every process checks full answers and retained output lifetime. Every scoped sum
fits inside outer preparation time; residual time includes assembly and teardown.

## Where preparation time goes

Median shares in the initial instrumented runs:

| Source | Admission/dictionaries | Relations | Ordering | Elimination |
|---|---:|---:|---:|---:|
| Equality star, 4/6 variables | 11–13% | 29–31% | 15–19% | 36–40% |
| Equality clique, 4 variables | 7–8% | 36% | 12% | 42% |
| Equality clique, 6 variables | 3% | 23% | 7–8% | 65–66% |
| Unequal-name star, 4/6 variables | 4–5% | 68–70% | 6–7% | 18–19% |

Shares are medians of per-process shares, so their rounded sum need not be one.
Residual assembly/teardown accounts for approximately 1–3%. These are attribution
measurements, not ordinary-build architecture timing verdicts.

## The repair and its check

All region domains are checked ground before local relations are built. A ground
name predicate is therefore an atom check; ground name disequality requires two
atoms with different terms. Neither needs name-variable discovery, structural
summary construction or a finite consistency solver. The adapter now evaluates
those two cases directly. Structural requirements retain their existing evaluator.

This simplifies the production path without adding another solver or assuming a
language restriction. Independent tests still cover compound non-names, shared
hidden coordinates, repeated predicates, contradiction, duplicate choices, aliases
and changing restrictions. All 28 source/projection/transport tests pass with
phase clocks disabled and enabled; scoped Clippy passes.

In the repaired profile, unequal-name relation construction is 8.6–8.8 µs for
four variables and 13.3–14.0 µs for six, compared with 45.7–45.8 µs and
74.2–74.6 µs initially. Its preparation share is now 28–30%; elimination is
42–45%. Equality-clique elimination remains approximately 65% at six variables.
The two profile batches are separate observations, not a paired ordinary timing
experiment; the complete-session conclusion from the previous pilot is unchanged.

Both batches retain their exact registered source snapshots and binary hashes.
The audit verifies all 480 terminal processes, their 240 exclusive phase sums and
frozen inputs. [Initial profile](s06-connected-preparation/analysis.json) ·
[Repaired profile](s06-connected-preparation-repair/analysis.json).

## Next decision

T076 next runs fresh complete ordinary timing and separate ownership measurements
against the frozen pre-repair projection and early-filtered enumeration. Keep the
same dense, multiplicity, alias, query-reuse and cancellation controls. This will
show whether removing theory compilation changes any complete comparison, rather
than merely producing a smaller component number.

Sparse elimination remains independently warranted by the dense profile. Ordered
source provenance and prefix/cancellation correspondence remain required before
using projected bags as raw source execution. Whole-call recognition remains an
alternative at the next breadth review. This is package three since that review;
the next package triggers the full portfolio reassessment. The goal is active.
