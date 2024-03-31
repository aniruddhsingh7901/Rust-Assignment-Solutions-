fn is_palindrome(s: &str) -> bool {
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}
