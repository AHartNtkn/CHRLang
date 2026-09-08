# S00 contract probe registration

Decision: which reference outputs and diagnostics may constrain new architecture comparisons?

This is a deterministic semantic experiment, not comparative timing. The source audit identifies reference FIFO/declaration order and failed/completed branch counts as implementation policy/accounting, while the abstract model permits committed scheduling and explicit alternatives. Test this distinction against the actual public reference API without changing its code.

Six independently predicted cases, each with 200 reference steps maximum:

| Case | Complete unique observations | Completed / duplicate / failed reference branches |
|---|---|---|
| Equal successful OR arms | one empty answer | 2 / 1 / 0 |
| Failure before an equal-arm OR in reference pending order | none | 0 / 0 / 1 |
| The same failure after that OR | none | 0 / 0 / 2 |
| Competing p→a and p→b, a first | residual a() | 1 / 0 / 0 |
| Same rules, b first | residual b() | 1 / 0 / 0 |
| Equal output terms with one versus two residual occurrences | two distinct complete answers | 2 / 0 / 0 |

All six must exhaust. These are hand-derived expected results, not outputs learned from another engine. Compare full output/residual objects, not only cardinalities. The last case prevents conflating duplicate alternatives with multiset resource multiplicity.

Run the example twice in separate release processes, each bounded by 60 seconds. Record toolchain and source revision. Compare deterministic output exactly; no warmups or timing inference are appropriate. Run the independent reference semantic suite as a separate coverage check. Any disagreement blocks interpretation and requires source/oracle investigation, not changing expected results to fit the engine.

Interpretation: differing failed-branch counts with the same completed observations show that those numbers distinguish these reference executions; they do not prove equivalence under every possible diagnostic API. Different answers for committed rule orders require matched-policy comparisons or independent allowed-execution arguments on nonconfluent sources. Equal-arm raw successes remain a separate diagnostic/semantic-family account from unique answer delivery. None of these results authorizes dropping source alternatives silently from a raw-multiplicity contract.
