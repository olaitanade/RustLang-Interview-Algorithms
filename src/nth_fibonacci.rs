
pub fn get_nth_fib(n:i64) -> i64{
    if n == 2{
        return 1;
    }else if n == 1 {
        return 0;
    }else {
        return get_nth_fib(n - 1) + get_nth_fib(n - 2);
    }
}