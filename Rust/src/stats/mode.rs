use std::collections::HashMap;

pub fn get_mode(list: &Vec<i32>) -> i32 {

    let mut counts_map = HashMap::new();

    for i in list {
        let count = counts_map.entry(i).or_insert(0);
        *count += 1;
    }

    *counts_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Calculate mode")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_mode_returns_mode_of_list() {
        let list = vec![1,2,3,4,4,5];

        assert_eq!(get_mode(&list), 4);
    }
}