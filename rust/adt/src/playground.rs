pub fn test() -> bool {
    let s = String::from("A man, a plan, a canal: Panama");
    let s: Vec<_> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    for i in 0..(s.len() / 2) {
        if s[i] != s[s.len() - 1 - i] {
            return false;
        }
    }
    true
}
