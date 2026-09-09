//! Diagnostic source-work screen; this binary makes no timing claim.
#[path = "../tests/join_support/mod.rs"]
mod fixture;
use chr_compiled::{Access, Policy, PreparedRuleset};
const LIMIT: usize = 20_000_000;
fn work(s: &chr_compiled::Stats) -> String {
    let mut work = Vec::new();
    macro_rules! counters { ($($name:ident),* $(,)?) => {$({work.push(format!("\"{}\":{}",stringify!($name),s.$name));})*}; }
    counters!(
        source_steps,
        candidate_visits,
        cursor_steps,
        history_checks,
        pool_visits,
        cursor_pool_entries,
        structural_tests,
        generic_ast_visits,
        binding_slot_copies,
        rule_dispatches,
        index_lookups,
        index_bucket_entries,
        index_inserts,
        index_removes,
        index_repairs,
        index_entries,
        max_index_entries,
        key_visits,
        key_template_visits,
        key_normalization_requests,
        key_normalization_allocations,
        activation_pushes,
        activation_coalesced,
        activation_pops,
        stale_activations,
        dependency_visits,
        dependency_refreshes,
        changed_variables,
        max_queue,
        max_occurrences,
        max_dependency_edges,
        dependency_edges,
        specialized_candidates,
        specialized_applications
    );
    work.join(",")
}
fn run(n: usize, requests: usize, policy: Policy, access: Access) -> Result<bool, String> {
    if !chr_compiled::COLLECT_METRICS
        || !chr_persistent::COLLECT_KERNEL_METRICS
        || !chr_observe::COLLECT_METRICS
    {
        return Err("work screen requires compiled, kernel and observer metrics".into());
    }
    let prepared = PreparedRuleset::new(fixture::rules(), None)?;
    let mut engine = prepared.start(fixture::query(n, requests), policy, access)?;
    let mut ticks = 0;
    let mut phases = Vec::with_capacity(requests);
    let mut completed_requests = 0;
    while ticks < LIMIT && !engine.status().exhausted {
        engine.step();
        ticks += 1;
        if completed_requests < requests
            && engine.stats().applications == 3 * (completed_requests + 1) as u64
        {
            completed_requests += 1;
            phases.push(format!("{{\"completed_requests\":{completed_requests},\"applications\":{},\"work\":{{{}}}}}", engine.stats().applications, work(engine.stats())));
        }
        if engine.status().pending_split {
            return Err("registered source unexpectedly split".into());
        }
    }
    let status = engine.status();
    let answer = engine.observe();
    let validated = answer
        .as_ref()
        .is_some_and(|answer| fixture::validate(answer, n, requests));
    let s = engine.stats();
    println!(
        "{{\"schema\":1,\"n\":{n},\"requests\":{requests},\"policy\":\"{}\",\"access\":\"{}\",\"metrics\":{},\"kernel_metrics\":{},\"observer_metrics\":{},\"complete\":{},\"failed\":{},\"tick_limit\":{LIMIT},\"ticks\":{ticks},\"validated\":{validated},\"validated_receipts\":{},\"applications\":{},\"work\":{{{}}},\"phases\":[{}]}}",
        if policy == Policy::Global {
            "global"
        } else {
            "active"
        },
        if access == Access::Scan {
            "scan"
        } else {
            "indexed"
        },
        chr_compiled::COLLECT_METRICS,
        chr_persistent::COLLECT_KERNEL_METRICS,
        chr_observe::COLLECT_METRICS,
        status.exhausted,
        status.failed,
        if validated { requests } else { 0 },
        s.applications,
        work(s),
        phases.join(",")
    );
    Ok(status.exhausted
        && !status.failed
        && validated
        && completed_requests == requests
        && s.applications == (3 * requests + n + 2) as u64)
}
fn main() {
    let result = (|| {
        let args = std::env::args().skip(1).collect::<Vec<_>>();
        if args.len() != 4 {
            return Err("usage: chr-join-screen N R global|active scan|indexed".into());
        }
        let n = args[0].parse::<usize>().map_err(|_| "invalid N")?;
        let requests = args[1].parse::<usize>().map_err(|_| "invalid R")?;
        if !(1..=4096).contains(&n) || requests > 4096 {
            return Err("N must be 1..4096; R must be 0..4096".into());
        }
        let policy = match args[2].as_str() {
            "global" => Policy::Global,
            "active" => Policy::Active,
            _ => return Err("invalid policy".into()),
        };
        let access = match args[3].as_str() {
            "scan" => Access::Scan,
            "indexed" => Access::Indexed,
            _ => return Err("invalid access".into()),
        };
        run(n, requests, policy, access)
    })();
    match result {
        Ok(true) => (),
        Ok(false) => std::process::exit(1),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_work_endpoint_or_explicit_metrics_rejection() {
        let result = run(2, 3, Policy::Active, Access::Indexed);
        if chr_compiled::COLLECT_METRICS {
            assert!(result.unwrap());
        } else {
            assert!(result.unwrap_err().contains("requires"));
        }
    }
}
