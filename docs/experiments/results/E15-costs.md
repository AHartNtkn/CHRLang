# E15 corrected service/session cost comparison

The release-before-validation comparison is pending. An independent audit found
that the first protocol collected validator garbage inside timed release. Its
[diagnostic data](E15-cost-diagnostic.md) remain available, but do not isolate the
intended cleanup cost.

The corrected harness releases search state before checking serialized answers,
then collects validation garbage outside measurement before the next query.
A regression test checks both boundaries; all 28 scheduling and 22 net tests pass.
An independent read-only review found no remaining blocking measurement defect.
The [v2 manifest](E15-cost-manifest-v2.json) includes the independent comparator and
postprocessing code as well as runtime/harness hashes. The same
135 configurations and repetition counts will be rerun under the revised
[registration](../registrations/E15.md). The broader research goal remains active.
