//! Explicit experimental source-executor selection.
use super::regions::Mode;
pub fn parse(value: &str) -> Result<(Mode, Option<usize>), String> {
    #[cfg(feature = "worker-lowering")]
    if value == "contracted" {
        return Ok((Mode::Contracted, None));
    }
    let workers = value
        .parse::<usize>()
        .map_err(|_| "expected worker count or enabled contracted mode")?;
    Ok((
        if workers == 0 {
            Mode::Inline
        } else {
            Mode::Threads(workers)
        },
        Some(workers),
    ))
}
