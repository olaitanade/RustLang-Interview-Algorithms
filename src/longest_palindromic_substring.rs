
//longest palindromic substring
pub fn longest_palindromic_substring(str_var: &str) -> &str {
    let mut longest: &str = "";
    for i in 0..str_var.len(){
        for j in i..str_var.len(){
            let substring: &str = &str_var[i..j + 1];
            if substring.len() > longest.len() && is_palindrome(substring) {
                longest = substring;
            }
        }
    }
    longest
}



//is palindrome
pub fn is_palindrome(str_var: &str) -> bool {
    let mut left_idx: usize = 0;
    let mut right_idx: usize = str_var.len() - 1;

    while left_idx < right_idx {
        if str_var.as_bytes()[left_idx] != str_var.as_bytes()[right_idx] {
            return false;
        }
        left_idx = left_idx + 1;
        right_idx = right_idx - 1;
    }
    true
}