#![allow(dead_code)]

use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input
        .par_iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}

fn increment_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(|p| *p += 1);
}

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let mid = partition(v);
    let (low, high) = v.split_at_mut(mid);
    rayon::join(|| quick_sort(low), || quick_sort(high));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let input = [1, 2, 3];
        let res = sum_of_squares(&input);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_inc() {
        let mut input = [1, 2, 3];
        increment_all(&mut input);
        assert_eq!(input, [2, 3, 4]);
    }

    #[test]
    fn test_join() {
        let mut v = vec![10, 9, 8, 5, 2, 3, 7, 6, 1, 4, 4];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_join_2() {
        let mut v = vec!["c", "a", "b"];
        quick_sort(&mut v);
        assert_eq!(v, vec!["a", "b", "c"]);
    }
}
