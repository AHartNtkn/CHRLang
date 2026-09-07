# Allocation ownership as an E16 follow-up

The equation comparison leaves a specific question: how much of the owned-worker
cost comes from transferable term representation, and how much from allocator
behavior when allocation and reclamation occur on different threads?

Local checks on 2026-09-07 report glibc 2.35 (`getconf GNU_LIBC_VERSION`) and show
`parallel_cost` dynamically linked to libc.so.6 (`ldd`). Its source explicitly selects
System. Rust documents System as using malloc and related functions on Unix; the
current documentation is background for that interface, not proof of the installed
compiler version, which remains recorded in the experiment manifest.
[System documentation](https://doc.rust-lang.org/std/alloc/struct.System.html).

The matching glibc 2.35 manual describes multiple malloc arenas supporting simultaneous
allocation by different threads, and separate mmap handling for large blocks. This
supports treating allocator behavior as a potential cost variable; it does not show
that allocator contention or remote reclamation caused the measured losses.
[GNU allocator](https://www.gnu.org/software/libc/manual/2.35/html_node/The-GNU-Allocator.html).
The complete page text was available through indexed search after direct fetch failed.

The owned interface exports operands on the owner, resolves/clones them on workers,
and returns owned substitutions for installation and eventual reclamation. The metered
heap counter sees requested layouts, not arena reservation or physical residency.
Therefore equal requested byte totals do not imply equal allocator/system storage
costs. This is an inference combining the local ownership path with allocator design,
not a measured causal attribution.

A useful experimental sequence is to first keep larger regional state on workers
(the independent coarse-region entry), then compare an optimized transferable term
representation under unchanged logical work. If allocator-sensitive differences remain,
register a same-work ownership/reclamation control and a narrowly justified allocator
configuration ablation. Keep the ordinary System baseline and record the exact runtime
library/configuration. Do not tune an allocator and attribute the gain to CHR semantics,
or use a service-only allocation test as a whole-query performance result.

This question is open. No unavailable resource or owner decision prevents a controlled
experiment; source research identifies a mechanism to distinguish rather than closing it.
