# Local nets: separate expressibility from cheap services

Nonoverlapping relation rules simplify dispatch, but do not by themselves make logical-variable services local. A finite encoded machine can supply those services; a distributed direct encoding must additionally solve routing and ownership. Restrictions that simplify those operations have specific programming costs worth comparing.

## A concrete finite-service route

A conservative service machine can encode logical-variable handles as finite natural numbers, bindings as a finite association list, constructor terms as tagged finite data, and pending equations as a worklist. The program uses a finite signature and finitely many control states for dereferencing, constructor comparison, decomposition, reachability, binding, wake-up and return. Dynamic identities are data, not infinitely many agent types or rewrite rules.

Its equality algorithm is the finite-tree worklist algorithm: scan the binding table to dereference each operand; compare roots; push matching children; for an unbound representative, traverse the other term's projected graph with a visited set before binding. Aliases use the same encoded handle. A private binding table is committed only after the source equation succeeds. A suspended demand records the traversed handles and must be rechecked on their changes.

Ordinary nets can implement finite-data computation through a universal encoding. That settles computability, not a direct, efficient service implementation. In this particular machine, linear table lookup visits up to V entries for V bindings; an occurs traversal may perform a lookup for each visited variable. The constructor-only part of an occurrence expansion may be local while equality is dominated by representation traversal. A full active-pair listing is still needed to establish the chosen compiler's exact simulation and cost; universality cannot supply those constants or a locality guarantee.

The proposed lower-complexity source fragment R eliminates partner search for its own single-headed occurrences. It does not eliminate the binding table, alias lookup, reachability, or demand wake-ups. That is the precise benefit and limitation of this compilation route, not a reason to discard it.

## Wire names are not unrestricted logical variables

Hassan, Mackie and Sato, *An Implementation Model for Interaction Nets* (2015), §§2–4, presents net names, indirection nodes, active-pair equations and a low-level representation. Its equation stack connects principal-port agents. [Paper](https://arxiv.org/pdf/1505.07164)

For this project, one logical variable can be referenced in arbitrarily many constraints. Ordinary net wiring has two endpoints. A fan or routing structure can mediate several references, but copying a variable's data must not create independent logical variables. Ground handle copying plus a shared service is a simple solution; direct variable cells require an explicit request protocol. The net calculus's use of the word “equation” is not evidence that the runtime already implements this unification contract.

A single principal port also means that concurrent requests for one variable need mediation. A queue can choose a definite permitted order. The compiler must preserve fairness and source atomicity across multi-variable operations; holding one variable while waiting for another can deadlock. A centralized equality service avoids that acquisition cycle at the cost of serialization. Distributed services need a global acquisition order, transaction protocol or nonblocking algorithm with its own progress proof.

## Three properties that can reduce equality cost

**Fresh construction at execution.** If t contains only constructors and handles proven unable to reach the unbound X at the moment of binding, X =:= t cannot introduce a cycle. An especially simple case constructs t from freshly allocated unbound children and binds X before exposing those children to other source operations. A compiler may fuse that sequence as a finite macro corresponding to consecutive permitted source steps.

Freshness at rule-body creation alone is insufficient. With pending equations `X =:= s(Y)` and `Y =:= X`, Y may have become aliased to X before the first equation is serviced. The compiler needs an execution-time certificate or a scheduling/ownership proof. Silently treating every rule-local variable as permanently fresh is unsound.

**Disjointness and linearity.** Drabent's extended occur-check report, Lemma 1.1, gives an NSTO condition for disjoint term-variable sets when one term sequence is linear. Its later mode criteria state selection-rule assumptions explicitly. [Paper](https://arxiv.org/pdf/2204.05379)

This suggests inference of local occurs-check certificates without changing finite-tree semantics. Apply the certificate to the current projected operands, including dereferenced aliases. Syntactic head linearity alone does not establish it for arbitrary delayed RHS equations. Requiring all programs to use fixed ground input modes would sacrifice relational calling modes; a per-operation certificate need not do that.

**Exclusive ownership.** A region may own a term graph and receive no external aliases while mutating it. This simplifies lifetime and synchronization. Ownership is different from source syntactic linearity: explicit immutable sharing can be allowed, while writable logical-variable aliases need controlled transfer. SK's repeated argument use means that a global ban on all sharing would impose a serious reformulation; an ownership discipline distinguishing immutable terms from logical-variable services is less restrictive but more complex.

## Source changes and their consequences

A language-wide single-headed, disjoint-pattern discipline makes every occurrence's rule selection local. Addition with a catch-all head and explicit RHS disjunction remains expressible in arbitrary modes. Multiheaded joins and propagation must be expressed through explicit state/services or excluded. This is a substantial departure from ordinary CHR, requiring an owner decision if adopted globally.

An inferred closed relation region offers the same dispatch opportunity locally. Multiheaded CHR remains outside it; shared variable ports need a service boundary. It requires a compiler analysis and cross-region protocol, but does not require programmers to duplicate forward and backward definitions.

An affine ownership discipline can simplify physical rewrites and reclaim cells when no alternative retains them. It cannot treat source duplication as erased work when an active duplicated constraint can fail or remains in the answer. The compiler must distinguish term references, logical-variable aliases and constraint occurrences.

Finite-domain annotations can make exact state tables and bitsets feasible, and can bound search at a particular call. Making all constructors or synthesis depth globally finite would exclude unbounded unary arithmetic and general program enumeration. Local bounds are a useful workload or solver option; a global finite language would change the stated target.

## Richer targets remain distinct candidates

Nested-pattern nets reduce administrative discrimination where their sequentiality conditions hold; they do not directly solve aliasable variables or arbitrary partner gathering. Multiport or additive extensions can express additional interactions, but their confluence, arbitration and cost properties must be stated for the selected extension.

Matsuoka's 1999 publisher abstract and the JFPLC 2001 conference abstract describe different extensions: the former reports nonconfluence in general, while the latter reports a confluent extension encoding additive proof nets. The latter is not evidence that every first-order-unification extension is confluent. The detailed 2001 rule system has not yet been obtained. [1999 record](https://ipsj.ixsq.nii.ac.jp/records/17012), [2001 author abstract at the conference](https://contraintes.inria.fr/jfplc2001/resumes.html)

Angelic CHR's head-decomposition technique does not transfer unchanged: intermediate consumption can strand resources and change residual answers. T015-source-expansion.md gives the counterexample. For this baseline, a richer graph compiler must instead commit an entire selected tuple or prove that partial gathering cannot interfere with any other observation.

## Disposition

The finite encoded service route is sufficiently concrete to explain why unrestricted variables are computable and where straightforward traversal costs arise. It does not constitute the promised direct net compiler. A full finite rewrite encoding, region interface, and administrative progress argument remain analytical work; the next step is to choose a small service representation for that derivation, not to assume the conditional-store prototype answers it.

The strongest current language recommendation is to investigate separate certificates for local dispatch, occurs-check elision, ownership and bounded resources. They buy different things. No evidence supports bundling them into one mandatory restriction or claiming that nonoverlap alone delivers efficient interaction-net compilation.
