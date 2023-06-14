#[cfg(test)]
mod test {
    fn bubble_sort(list: &mut Vec<i32>) -> Vec<i32> {
        let mut temp: i32;

        for i in 0..list.len() {
            for j in 0..(list.len() - i) {
                let mut left_value = list.get_mut(j as usize).expect("missing index");
                let mut right_value = list.get_mut((j + 1) as usize).expect("missing index");
                if *left_value > *right_value {
                    temp = *left_value;
                    *left_value = *right_value;
                    *right_value = temp;
                }
            }
        }

        list.to_vec()
    }
}
