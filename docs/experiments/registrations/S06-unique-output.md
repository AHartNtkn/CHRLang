# Challenge union with single explicit traversal and output collection

The reuse comparison favors union on repeated overlapping full output, but its explicit control traverses branches separately. Retained output also accounts for most of union's adverse retained-all peak. Test those two mechanisms independently before further confirmation.

## Algorithms and correctness

Add `unique`: normalize/deduplicate source branches as in the existing `dedup` control, then traverse complete assignments once. A partial assignment is viable when at least one branch has no violated constraint. This predicate may allow an eventually impossible prefix; it must not reject a satisfiable extension. The existing fixed-coordinate traversal gives unique lexicographic complete rows without retained union compilation.

Compare four modes: union, unique, deduplicated branchwise explicit, and reduced disjunction. All operate on the same finite logical assignment-set contract. Two collectors form a factorial contrast. `set` retains the existing BTreeSet. `ordered` collects vectors, sorting/deduplicating only for modes that can emit duplicates; union and unique append their already unique ordered rows. Shrink outer vectors to their final length in every ordered mode, charging any reallocation. Report discovery and collection effects separately. This changes the physical representation of the same final logical set, not raw CHR multiplicity semantics or an adopted language interface.

Before comparative runs, test both collectors against independent complete sets on widths four/eight, four families and five callers. Prove the unique visitor's strict order, exact first row and early stopping; check retained ordered outputs after preparation disposal. Preserve the failing missing-mode and missing-collector tests, then their passing results. Every measured process additionally preflights its complete session and full-output internal visitor cancellation/prepared reuse. Branchwise internal stopping is not first delivery from a canonical ordered collector, which must finish deduplication first.

## Prospective sizing matrix

Width eight, three names; four qualified families (overlap, disjoint, redundant, single); one/64 queries; membership/full output; immediate/two-answer-window/all consumers; four modes; both collectors. This is 384 allocation cells, each repeated twice: 768 processes, shuffled seed 76041. Retained-all ordinary timing has 128 cells, five repetitions each: 640 processes, shuffled seed 76042. Every process uses the existing changing-caller cycle, complete independent oracle and warmup. Record toolchain before builds, source/binary freeze before comparative runs and five clock checks before timing. Freeze fresh binaries without overwriting earlier experiments.

Run width-four/eight-query/full/window smoke for every mode, family and collector before freezing (32 processes). Pin measured processes to the lowest permitted CPU. Each process has 60 seconds wall/CPU, 1 GiB memory and the existing diagram budget; builds have 300 seconds and the matrix 30 minutes. Preserve failures and diagnose cutoffs.

Use metrics-off allocation and ordinary builds separately. Preserve the lifecycle phases, baseline restoration and retained-answer checks of the preceding registration. Execution/complete observation remain fused; first-delivery latency, native compilation, process startup, oracle fixtures and fixed recording/consumer containers are outside this measurement. Count vector construction, sorting/deduplication, shrinkage and all output disposal inside phases. No complete-architecture superiority follows.

## Interpretation

Hypothesis: single traversal can obtain part of union's duplicate-avoidance benefit without its preparation. It may instead spend more on repeatedly checking viable alternatives. Ordered collection may reduce output ownership independently; branchwise sort/deduplication may introduce different costs. Keep low reuse, disjoint, redundant and single-branch sources as adverse contrasts. Membership is a negative collector control: it uses the same boolean endpoint in both collector modes and should have identical allocation phases.

Require exact normalized allocation repeats. Compare matched cells and collectors; separate traffic, peak and retained output after preparation disposal. Five timing samples remain exploratory. Show median and full matched-repeat ratios, retaining all samples; flag any compared total below 100 times the maximum median empty-clock cost times the number of phase intervals. A ten-percent difference selects further investigation, not statistical significance. Do not pool sources into invented workload weights.

At the result, attribute any change in union's favorable regime to discovery versus collection. Compare consequential repair/confirmation with structural-prefix generated/access controls and unfinished demand/integration costs. This is the first package after the full reuse portfolio review. Broader compact operations and source/language eligibility remain open.
