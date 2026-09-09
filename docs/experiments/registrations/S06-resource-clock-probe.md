# Clock-window diagnostic after the resource-fusion pilot

The1350-process pilot passes, but short phase CPU totals exceed wall totals materially. The CPU interval encloses the wall-clock calls, so zero wall/CPU scheduling flags cannot establish an undistorted clock. Run5000 empty closures with the same clock-call order in an optimized standalone Rust probe. Save every wall/CPU pair and report medians/ranges. This is approximate measurement-overhead evidence, not a correction to subtract from workload samples. Preserve all pilot observations; no new speed classification follows.
