# Live caller integration for compressed private traces

Register before implementation. Suspend a settled caller only when its next
applicable rule belongs to a private single-head priority prefix and exactly one
private occurrence is live. Remove that occurrence, skip the isolated insertion
step, and replay each private source event through the caller's FIFO queue.
At the private completion event, import bindings/residual private work and service
the caller in that same tick. Private suspension must resume caller execution,
not become failure or a cached empty answer. Keep occurrence order among live
caller facts, fresh-variable separation and caller propagation history.

Check whole sources with unequal branch lengths, observable output bindings,
competing token consumers, retained propagation history, fresh aliases, early
failure, and a private call suspended until a caller supplies its input. Compare
every delivered prefix/exhaustion with Direct and complete answers with independent
scalar execution. Cancel at 0/1/5 steps and restart the same preparation. Check
cross-owner misuse. Bound each run to 100,000 service steps; test both counter
configurations and scoped Clippy. No performance verdict from this gate.

The priority-prefix and ownership check is an optimizer admission premise, not a
language restriction. Sources with higher-priority observers need their own
interleaving protocol; do not claim these run through the trace path. At this gate,
select further boundary work versus complete costs and sparse projection by the
actual behavior and saved work established.
