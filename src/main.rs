//mod longest_palindromic_substring; 
//mod nth_fibonacci;
//mod two_number_sum;
mod validate_subsequence;

fn main() {
    //println!("longest palindromic substring = {}",longest_palindromic_substring::longest_palindromic_substring("ab12365456321bb"));
    //println!("nth fibonacci = {}",nth_fibonacci::get_nth_fib(5));

    //let sum_array: [i32; 8] = [3, 5, -4, 8, 11, 1, -1, 6];
    //let target_sum: i32 = 10;
    //println!("Two number sum array = {:?}, target sum = {}, answer = {:?}",sum_array,10,two_number_sum::two_number_sum( &sum_array, target_sum));

    let array: [i32; 8] = [5, 1, 22, 25, 6, -1, 8, 10];
    let sequence: [i32; 4] = [1, 6, -1, 10];

    println!("Is array a subsequence : {}",validate_subsequence::is_valid_subsequence(&array, &sequence));

}