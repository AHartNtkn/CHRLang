# T044 structural index-maintenance causal gate

Prospective deterministic work gate, not a timing comparison. R08's one-region carry has equal source applications but large elapsed/allocation differences. Test whether the current compiled control repeatedly traverses immutable suffixes for dependencies and canonical index keys.

Use one source rule `carry(X,s(N)) <=> carry(X,N)`, query `carry(a,s^n(z))`, depths 8,16,32. Independently expect exactly one complete raw answer with residual `carry(a,z)` and n applications. Test ordinary and inferred-specialized execution with Global/Scan and Global/Indexed; preserve exact source occurrence trace across access modes. Existing counters predict `(n+1)(n+4)/2` dependency visits in both access modes, that many key visits and normalization requests in Indexed, zero new normalization nodes, `2(n+1)` index insertions and `2n` removals. Scan supplies mechanism attribution, not a proposed substitute for indexing. Keep diagnostics-off semantic runs separate; they make no counter prediction.

The adverse source has a pending `p(f(X), closed_suffix)`, an alias binding X=Y, and a later binding Y=a. A consuming match for `p(f(a), closed_suffix)` must become enabled. Check full nonground residual aliases and exact rule sequence under Global and Active policies, ordinary and inferred-specialized execution, Scan and Indexed. This prohibits ignoring variables merely because a term also contains immutable children.

Bound each query to one million search ticks; reject unexpected source splits/failures, duplicate answers or incomplete exhaustion. No elapsed/allocation ranking or runtime optimization is part of this gate. If the predicted structural work is established, assess a bounded immutable-node property against binding-sensitive caching before selecting a correction. Preserve dynamic dependency repair and index lookup obligations. Static-lowering eligibility remains the strongest broader alternative if the maintenance is necessary or correction entails substantial redesign.


## Entry corrections from the first gate run

The first run contradicts the Scan dependency prediction: Global/Scan does not require watchers and records zero dependency visits. `needs_dependencies` intentionally omits that maintenance. The Indexed prediction remains unchanged; Scan is expected to record zero dependency and key visits. The existing sealed specialization also explicitly requires Global policy. Its Active configuration is therefore checked for rejection, while ordinary Active executes the semantic witness. These observations refine configuration applicability; they do not weaken Indexed repair or source-answer requirements. The first-run log is retained with the result.
