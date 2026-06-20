pub fn split_and_double(xs: &mut Vec<i32>, mid: usize) -> (&mut [i32], &mut [i32]) {
    let (left, right) = xs.split_at_mut(mid);
    for x in left.iter_mut() {  // can not use for x in left because it is a mutable borrow, we need to use iter_mut to get mutable references
        *x *= 2;
    }
    for x in right.iter_mut() {
        *x *= 2;
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_four_elements() {
        let mut xs = vec![1, 2, 3, 4];
        {
            let (left, right) = split_and_double(&mut xs, 2);
            assert_eq!(left, &mut [2, 4]);
            assert_eq!(right, &mut [6, 8]);
        }
        assert_eq!(xs, vec![2, 4, 6, 8]);
    }

    #[test]
    fn mid_zero_empty_left() {
        let mut xs = vec![1, 2, 3];
        {
            let (left, right) = split_and_double(&mut xs, 0);
            assert!(left.is_empty());
            assert_eq!(right, &mut [2, 4, 6]);
        }
        assert_eq!(xs, vec![2, 4, 6]);
    }

    #[test]
    fn mid_equals_len_empty_right() {
        let mut xs = vec![1, 2, 3];
        {
            let (left, right) = split_and_double(&mut xs, 3);
            assert_eq!(left, &mut [2, 4, 6]);
            assert!(right.is_empty());
        }
        assert_eq!(xs, vec![2, 4, 6]);
    }

    #[test]
    fn empty_vec() {
        let mut xs: Vec<i32> = vec![];
        {
            let (left, right) = split_and_double(&mut xs, 0);
            assert!(left.is_empty());
            assert!(right.is_empty());
        }
        assert_eq!(xs, Vec::<i32>::new());
    }

    #[test]
    fn single_element_mid_one() {
        let mut xs = vec![5];
        {
            let (left, right) = split_and_double(&mut xs, 1);
            assert_eq!(left, &mut [10]);
            assert!(right.is_empty());
        }
        assert_eq!(xs, vec![10]);
    }

    #[test]
    fn even_split_in_half() {
        let mut xs = vec![10, 20, 30, 40];
        {
            let (left, right) = split_and_double(&mut xs, 2);
            assert_eq!(left, &mut [20, 40]);
            assert_eq!(right, &mut [60, 80]);
        }
        assert_eq!(xs, vec![20, 40, 60, 80]);
    }

    #[test]
    fn odd_length_split() {
        let mut xs = vec![1, 2, 3];
        {
            let (left, right) = split_and_double(&mut xs, 1);
            assert_eq!(left, &mut [2]);
            assert_eq!(right, &mut [4, 6]);
        }
        assert_eq!(xs, vec![2, 4, 6]);
    }

    #[test]
    #[should_panic]
    fn panics_when_mid_exceeds_len() {
        let mut xs = vec![1, 2];
        let _ = split_and_double(&mut xs, 99);
    }
}
