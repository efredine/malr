use lazy_static::lazy_static;
use regex::{CaptureMatches, Match, Regex};

// Return an iterator on the match for two reasons:
// 1. no additional memory allocations;
// 2. retaining the position of the match in the text will be useful for error reporting.
pub fn tokenize(text: &str) -> impl Iterator<Item = Match> {
    // Using lazy static for the regex accomplishes two things:
    // 1. The regex is only compiled once;
    // 2. There are no lifetime issues with returning an iterator on the captured results.
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
                .unwrap();
    }

    // There will only ever be one capture group for each match because that's how the regex
    // is constructed. There should be one match for each type of token. The call to capture.get
    // returns an Option<Match> and filter_map filters out the None values.
    RE.captures_iter(text).filter_map(|capture| capture.get(1))
}
