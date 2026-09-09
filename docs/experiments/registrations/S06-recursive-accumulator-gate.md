# Source-derived recursive constructor updates

The existing carrier compiler already contracts known unary prefixes with unknown-tail suspension, singleton admission, exact occurrence identities and ordinary terminal arbitration. Reuse that mechanism. The new distinction is a recursive step that constructs or rearranges carried arguments instead of passing them unchanged.

Infer a single removed-head step whose control is a unary constructor over a distinct variable, with distinct variable patterns in other positions. Require its body to be exactly one recursive insertion: control strictly descends to that child; other arguments may be constructor expressions over head variables only. No new variables, equations, guards, choices, resource claims or extra heads are admitted in the step. Preserve the existing terminal-head and singleton rules. Terminal bodies and later bindings remain ordinary source execution.

Compile the carried-argument update from the prepared source. During primary contraction, perform those constructor updates while inspecting one known control node per service tick. At an unknown/malformed/terminal tail, publish the corresponding occurrence and resume ordinary arbitration. Trace/audit execution must replay ordinary commits without speculative constructor allocation. Unchanged passthrough steps retain their existing path.

Validate against independent scalar source execution and ordinary specialized execution: renamed predicates/constructors, nested updates, unknown tails with later binding, joint aliases and constructor failure, terminal choices and consuming effects, multiple carriers, fresh-variable rejection, intermediate observers and finite siblings. Require actual nonzero contracted steps for the new eligible recursive updates and exact occurrence identities/diagnostic traces. Check interruption and resumed query independence. No speed claim before lifecycle registration.

This extends an existing source-derived runtime loop; it is not a second baseline, a native code generator or general effectful recursion. The recursive mechanism package remains in progress until the new source gate and four-package breadth review are complete.
