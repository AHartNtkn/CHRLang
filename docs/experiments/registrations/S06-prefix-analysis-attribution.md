# Attribute repeated prefix eligibility analysis

Confirmed cold length32/single-call cases lose against sealed execution, while length32/eight-query cases gain. Preparation is material: plain1 with consumption has median183 microseconds of prefix analysis versus53 microseconds of sealed preparation in the eight-query cells. The compiler repeatedly visits overlapping dependency prefixes and clones successive candidate maps.

Replace repeated DFS with memoized dependency bounds, retaining exactly the same largest closed acyclic prefix. Keep expansion, freshening, entry generation, target preparation, workload and observation unchanged. Validate prefix selection against an independent bounded graph oracle and all semantic/trace gates before builds.

Preserve the frozen original binaries. Compare before-lowered, after-lowered and unchanged sealed execution on all length32 cases: four families, one/eight queries, consumption false/true (16 configurations). One warmup plus seven shuffled measured blocks, seed7303+block, yields384 ordinary processes. Run after allocation twice per configuration (32 processes), comparing with existing original diagnostics. Retain lifecycle endpoints, source validation, CPU/resource bounds and the median10%/all-pairs-direction criterion. Compare after/before and after/sealed within blocks. This intervention changes preparation only.

## Short-chain recheck

The length32 intervention materially reduces preparation and changes some cold losses to unresolved comparisons. To establish contrary cases for the corrected compiler, recheck all length1 configurations (four families, one/eight queries, consumption false/true) with after-lowered versus unchanged sealed. Use one warmup plus seven shuffled blocks, seed7304+block (256 ordinary processes), and two after allocation runs per configuration (32 processes). Same endpoints, bounds and practical criterion. This is registered before those runs; length8 remains covered only by the original compiler's full matrix.
