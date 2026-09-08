# Selective discovery pilot: script ownership obscured the join comparison

The first 224-process lifecycle pilot found no Retained/Direct difference meeting the registered 20% practical threshold. Both lowerings lost to generic execution on consumption and broad replacement, but phase analysis traced much of that cost to quadratic script copying during setup. The [paired ownership correction](S01-script-ownership-repair.md) provides the current interpretation of these comparisons.

All 224 processes completed with correct full answers, and all 32 allocation cells replayed exactly. The [prospective registration](../registrations/S01-selective-lifecycle.md) covers eight stable/update/consumption families, four execution configurations and eight changing queries over prepared source. [Raw records](s01-selective-lifecycle/raw.jsonl), [summaries](s01-selective-lifecycle/summary.csv) and [source/binary freeze](s01-selective-lifecycle/freeze.json) preserve this initial result.

## The consequential finding

On selective consumption, direct lowering spent a median 1.868 ms in setup out of 2.912 ms total, versus 0.200 ms setup out of 1.186 ms for generic scanning. Broad replacement spent 3.332 ms in direct setup out of 5.152 ms total. Retained setup showed the same pattern. The similar cost in both lowerings is not evidence that pair maintenance intrinsically costs that much.

The decoder cloned the entire remaining instruction suffix twice at each step: once to preserve a possible blocked-driver residual, and once to continue parsing. For a script of L constant-size instructions, that constructs quadratic syntax even though source execution requires only one instruction spine. Long update scripts exposed this ownership choice more strongly than short stable scripts.

This result selected a narrowly attributable correction: retain one source script, decode by reference and remember instruction positions. A blocked replacement still must reconstruct its exact unprocessed source residual; removing that observation requirement would invalidate the experiment. The correction received its own prospective paired registration and fresh control measurements.

## Limits of the initial matrix

Retained/direct paired medians ranged from 0.898 to 1.094 across complete sessions. Those values neither establish a practical retention benefit nor resolve the broader request-frequency crossover. Their shared script cost can dilute an execution difference. Likewise, the initial lowering/generic losses must not be treated as final architecture rankings after identifying that shared defect.

Timing was counter-free with the ordinary allocator, and requested-allocation diagnostics used a separate build. Source construction, input cloning, independent validation, native compilation and process startup were outside measurement. Preparation, setup, execution, observation and disposal were included. No failed or unfinished endpoint was counted as a completed cost.
