type Branch = Vec<(usize, usize)>;
pub fn family(n: usize, name: &str) -> Vec<Branch> {
    match name {
        "disjoint" => [(0, 1), (2, 3)]
            .iter()
            .map(|skip| {
                let mut edges = (0..4)
                    .flat_map(|i| (i + 1..4).map(move |j| (i, j)))
                    .filter(|e| e != skip)
                    .collect::<Vec<_>>();
                edges.extend((4..n).map(|i| (i - 1, i)));
                edges
            })
            .collect(),
        "overlap" => (0..n)
            .map(|skip| {
                (0..n)
                    .filter(|i| *i != skip)
                    .map(|i| (i, (i + 1) % n))
                    .collect()
            })
            .collect(),
        "redundant" => vec![(0..n).map(|i| (i, (i + 1) % n)).collect(); n],
        "single" => vec![(0..n).map(|i| (i, (i + 1) % n)).collect()],
        _ => panic!("unknown family"),
    }
}
#[allow(dead_code)]
pub fn families(n: usize) -> Vec<(&'static str, Vec<Branch>)> {
    ["overlap", "disjoint", "redundant", "single"]
        .into_iter()
        .map(|name| (name, family(n, name)))
        .collect()
}
