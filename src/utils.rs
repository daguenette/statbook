pub(crate) fn to_dash_case(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
        .to_lowercase()
}

