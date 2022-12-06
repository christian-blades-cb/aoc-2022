pub fn part1(xs: &str) -> usize {
    let max = xs.len();
    (0..max - 4)
        .map(|i| (i + 4, xs.get(i..i + 4).unwrap()))
        .filter(|(_idx, marker)| {
            let ys: Vec<char> = marker.chars().collect();
            ys[0] != ys[1] && ys[0] != ys[2] && ys[0] != ys[3] && // 0 != 1..=3
	    ys[1] != ys[2] && ys[1] != ys[3] && // 1 != 2..=3
	    ys[2] != ys[3]
        })
        .next()
        .unwrap()
        .0
}

pub fn part2(xs: &str) -> usize {
    let max = xs.len();
    const MARKER_LEN: usize = 14;
    (0..max - MARKER_LEN)
        .map(|i| (i + MARKER_LEN, xs.get(i..i + MARKER_LEN).unwrap()))
        .filter(|(_idx, marker)| {
            let ys: Vec<char> = marker.chars().collect();
            (0..MARKER_LEN - 1)
                .flat_map(|i| (i + 1..MARKER_LEN).into_iter().zip([i].into_iter().cycle()))
                .all(|(a, b)| ys[a] != ys[b])
        })
        .next()
        .unwrap()
        .0
}
