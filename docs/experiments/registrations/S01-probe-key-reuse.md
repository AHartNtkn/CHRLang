# Selective probe: reuse the already selected key

The first frozen qualification passes semantics but exposes duplicated key searches: the neutral size-128 Indexed/Global control uses 1,024 index lookups, while the feature uses 1,536 without starting a probe. The wrapper computes the current key, then ordinary pool construction computes it again.

Factor pool materialization from key selection and reuse the selected current/later key. Keep default semantics and source order, cardinality eligibility and all probe counters unchanged. Rerun exactly the preceding 18-process source/work qualification under a fresh freeze, same ordinary allocator, CPU/memory/time bounds and complete oracle cases. Require default work rows to match the first freeze exactly and neutral feature rows to remove duplicate searches. Genuine extra lookups deciding probe eligibility remain charged. This repairs the observed overhead; it does not establish timing, allocation or bounded service costs.
