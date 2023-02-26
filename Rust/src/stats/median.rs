pub fn get_median(list: &Vec<i32>) -> f32 {
    let length = list.len();

    if length % 2 == 0 {
        let middle_index = (length / 2) - 1;
        let middle_value = list[middle_index as usize];
        let next_middle_value = list[(middle_index + 1) as usize]; 
        return (middle_value + next_middle_value) as f32 / 2.0;
    }

    let middle_index = (length - 1) / 2;

    list[middle_index as usize] as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_median_returns_median_of_list_with_even_number_of_elements() {
        let list = vec![1,2,3,4,5,6];

        assert_eq!(get_median(&list), 3.5);
    }

    #[test]
    fn get_median_returns_median_of_list_with_odd_number_of_elements() {
        let list = vec![1,2,3,4,5];

        assert_eq!(get_median(&list), 3.0);
    }
}
