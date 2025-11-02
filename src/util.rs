use regex::Regex;

//
// Returns true if a string consists of multiple parts separated by '-'.
// String does not use any other separator.
//
pub(crate) fn is_subdivided_by_dashes_only(s: &str) -> bool {
    // Return false if string is empty
    if s.is_empty() {
        return false;
    }

    // Return false if there are invalid separators present
    let forbidden_separators = [' ', '_', ',', ';', '.', '/', '\\', ':', '|', '+'];
    if s.chars().any(|ch| forbidden_separators.contains(&ch)) {
        return false;
    }

    // If splitting by dash gives more than one part, it's separated by dashes
    // and no other separators are present
    s.split('-').count() > 1
}

//
pub fn contains_any_word(haystack: &str, words: &[&str]) -> bool {
    let pattern = words
        .iter()
        .map(|w| regex::escape(w))
        .collect::<Vec<_>>()
        .join("|");
    let regex_pattern = format!(r"\b({})\b", pattern);
    let re = Regex::new(&regex_pattern).unwrap();
    re.is_match(haystack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subdivided_by_dashes_only() {
        let label1 = "11-24h2-iot-lts";
        let label2 = "11-24h2-iot lts";

        assert_eq!(is_subdivided_by_dashes_only(label1), true);
        assert_eq!(is_subdivided_by_dashes_only(label2), false);
    }

    #[test]
    fn test_contains_any_word_true() {
        let label1 = "Windows 11 Professional Edition";
        let label2 = "Windows 11 Pro 24H2";
        let word = ["Professional Edition", "Professional", "Pro"];

        assert_eq!(contains_any_word(label1, &word), true);
        assert_eq!(contains_any_word(label2, &word), true);
    }

    #[test]
    fn test_contains_any_word_false() {
        let label1 = "Windows 11 Professional Edition";
        let label2 = "Windows 11 Pro 24H2";
        let word = ["Enterprise Edition", "Enterprise"];

        assert_eq!(contains_any_word(label1, &word), false);
        assert_eq!(contains_any_word(label2, &word), false);
    }
}
