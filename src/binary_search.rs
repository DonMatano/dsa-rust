#[cfg(test)]
mod test {
    use std::{cmp::Ordering, vec};

    fn binary_search(list_of_nums: Vec<usize>, search_value: usize) -> Option<usize> {
        let mut low: usize = 0;
        let mut high: usize = list_of_nums.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if let Some(mid_value) = list_of_nums.get(mid) {
                match &mid_value.cmp(&search_value) {
                    Ordering::Equal => return Some(mid),
                    Ordering::Greater => {
                        if mid == 0 {
                            return None;
                        }
                        high = mid - 1
                    }
                    Ordering::Less => low = mid + 1,
                }
            }
        }
        None
    }
    #[test]
    fn test1() {
        let nums: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Some(2), binary_search(nums, 3));
    }
}
