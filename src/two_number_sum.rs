
pub fn two_number_sum(array_obj: &[i32], target_sum: i32) -> Vec<i32> {
    for i in 0..array_obj.len(){
        let first_num: i32 = array_obj[i];
        let temp_i: usize = i + 1;
        for j in temp_i..array_obj.len(){
            let second_num: i32 = array_obj[j];
            if first_num + second_num == target_sum {
                return vec![first_num, second_num];
            }
        }
    }
    vec![]
}