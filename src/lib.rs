use itertools::Itertools;

fn order_weight(s: &str) -> String {
    match s.len() {
        0 => "".to_string(),
        _ =>  s.split_ascii_whitespace()
                .filter(|&s| !s.is_empty())
                .sorted()
                .sorted_by_key(|&x| x.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>())
                .collect::<Vec<_>>()
                .join(" "),
    }
}
