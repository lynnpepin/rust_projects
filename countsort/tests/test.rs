// TODO!
// god it is so hard to make packages work

#[cfg(test)]
mod countsort_tests {
    use super::*;
    #[test]
    fn this_should_pass() { 
        assert_eq!(3, 3);
    }

    fn test_a_sort() {
        let vals: Vec<u32>  = vec![2, 0, 4, 5, 7, 7, 0, 6, 1, 7, 6];
        let w: u8 = 3;
        let sorted_vals = count_and_sort(vals, w);
        let test_vals: Vec<u32> = vec![0, 0, 1, 2, 4, 5, 6, 6, 7, 7, 7];
        assert_eq!(vals, test_vals);
    }
}