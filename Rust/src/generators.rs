use rand::Rng;

pub fn generate_sorted_list(range: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();

    let mut list = Vec::new();

    for _ in 0..range {
        list.push(rng.gen_range(0..10))
    }
    
    list.sort();

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sorted_list_generates_list_from_range() {
        let range = 3;
        let list = generate_sorted_list(range);

        assert_eq!(list.len() as i32, range);
    }
}