fn main() {
    let mut arr = vec![1];
    quick_sort(&mut arr);
    println!("{:?}", arr);
}

fn partition(arr: &mut Vec<i32>, mut l: usize, mut r: usize) -> usize {
    let pivot = arr[l];
    while l < r {
        while l < r && arr[r] >= pivot {
            r -= 1;
        }
        arr[l] = arr[r];
        while l < r && arr[l] <= pivot {
            l += 1;
        }
        arr[r] = arr[l];
    }
    arr[l] = pivot;
    l
}

fn quick_sort_range(arr: &mut Vec<i32>, l: usize, r: usize) {
    if l < r {
        let p = partition(arr, l, r);
        quick_sort_range(arr, l, p);
        quick_sort_range(arr, p + 1, r);
    }
}

pub fn quick_sort(arr: &mut Vec<i32>) {
    if arr.len() > 0 {
        quick_sort_range(arr, 0, arr.len() - 1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_empty_array() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_quick_sort_single_element() {
        let mut arr = vec![1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn test_quick_sort_sorted_array() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse_sorted_array() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_random_array() {
        let mut arr = vec![3, 6, 8, 10, 1, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 6, 8, 10]);
    }

    #[test]
    fn test_quick_sort_array_with_duplicates() {
        let mut arr = vec![5, 1, 5, 1, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 5, 5, 5]);
    }
}