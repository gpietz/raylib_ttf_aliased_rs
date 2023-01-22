/// Removes the first and last char of a given string sequence.
pub(crate) fn remove_first_last_char(s: &str) -> Option<String> {
    if s.len() <= 2 { return None }
    let mut chars = s.chars();
    let _ = chars.next();
    let _ = chars.next_back();
    Some(chars.collect())
}
