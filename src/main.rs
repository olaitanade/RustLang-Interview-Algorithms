//mod longest_palindromic_substring; 
//mod nth_fibonacci;
mod two_number_sum;

fn main() {
    //println!("longest palindromic substring = {}",longest_palindromic_substring::longest_palindromic_substring("ab12365456321bb"));
    //println!("nth fibonacci = {}",nth_fibonacci::get_nth_fib(5));

    let sum_array: [i32; 8] = [3, 5, -4, 8, 11, 1, -1, 6];
    let target_sum: i32 = 10;
    println!("Two number sum array = {:?}, target sum = {}, answer = {:?}",sum_array,10,two_number_sum::two_number_sum( &sum_array, target_sum));
}