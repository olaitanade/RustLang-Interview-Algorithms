
pub fn is_valid_subsequence(array: &[i32], sequence: &[i32]) -> bool {
    let mut seq_idx: usize = 0;
    for val in array.iter() {
       if seq_idx == sequence.len() {
         break;
       }

       if sequence[seq_idx] == *val {
           seq_idx = seq_idx + 1;
       }
    }
    seq_idx == sequence.len()
}