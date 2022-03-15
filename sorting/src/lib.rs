/*                                      Notes
    Loop invariants:
        1. At the start of each iteration of the outer loop, A[0..i] contains a sorted subset of the 
        original array.

        2. Every element a in A[0..i] is smaller/equal than any element in A[i+1..n]

    Best  case: O(n^2)
    Worst case: O(n^2)

    Notes:
        Only need to check n-1 because the final element is already larger than any prior element
        according to invariant #2.
*/
/// Loop through array, find smallest input and swap arr[0] with it. Proceed with second smallest 
/// input and arr[1], etc
pub fn selection_sort(arr: &mut [usize]) {
    if arr.len() == 0 { return; }

    for i in 0..arr.len()-1 {
        let mut tmp = i;
        for j in i+1..arr.len() {
            if arr[tmp] > arr[j] {
                tmp = j;
            }
        }
        arr.swap(i, tmp);
    }
}

/*
   TODO
*/
/// Loop through array, and propagate each element downwards until it reaches its correct spot in
/// the array
pub fn insertion_sort(arr: &mut [usize]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j-1] > arr[j] {
            arr.swap(j-1, j);
            j -= 1;
        }
    }
}

/*
   TODO
*/
/// Split the array into 2 parts, sort each part individually and then merge the 2 parts while 
/// maintaining the sorted order for the result. The individual sorting is done recursively.
pub fn merge_sort(arr: &[usize]) -> Vec<usize> {
    let merge = |l: &[usize], r: &[usize]| -> Vec<usize> {
        let mut res = Vec::new();
        let (mut il, mut ir) = (0, 0);

        loop {
            if il == l.len() { res.extend(&r[ir..]); break; }
            if ir == r.len() { res.extend(&l[il..]); break; }

            if l[il] < r[ir] {
                res.push(l[il]);
                il += 1;
            } else {
                res.push(r[ir]);
                ir += 1;
            }
        }
        res
    };

    if arr.len() <= 1 { return arr.to_vec(); }

    let mid: usize = arr.len() / 2;
    let left = merge_sort(&arr[0..mid]);
    let right = merge_sort(&arr[mid..arr.len()]);
    merge(&left, &right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted() {
        let orig = vec![1, 2, 3, 4, 5];
        let mut arr;

        arr = orig.clone();
        selection_sort(&mut arr);
        if !matches!(arr[..], [1, 2, 3, 4, 5]) {
            panic!("selection_sort-already_sorted: {:?}", arr);
        }

        arr = orig.clone();
        insertion_sort(&mut arr);
        if !matches!(arr[..], [1, 2, 3, 4, 5]) {
            panic!("insertion_sort-already_sorted: {:?}", arr);
        }

        if !matches!(merge_sort(&orig)[..], [1, 2, 3, 4, 5]) {
            panic!("merge_sort-already_sorted: {:?}", arr);
        }
    }

    #[test]
    fn small_test1() {
        let orig = vec![2, 3, 1, 5, 4];
        let mut arr;

        arr = orig.clone();
        selection_sort(&mut arr);
        if !matches!(arr[..], [1, 2, 3, 4, 5]) {
            panic!("selection_sort-small_test1: {:?}", arr);
        }

        arr = orig.clone();
        insertion_sort(&mut arr);
        if !matches!(arr[..], [1, 2, 3, 4, 5]) {
            panic!("insertion_sort-small_test1: {:?}", arr);
        }

        if !matches!(merge_sort(&orig)[..], [1, 2, 3, 4, 5]) {
            panic!("merge_sort-small_test1: {:?}", arr);
        }
    }

    #[test]
    fn small_test2() {
        let orig = vec![5, 2, 4, 6, 1, 3];
        let mut arr;

        arr = orig.clone();
        selection_sort(&mut arr);
        if !matches!(arr[..], [1, 2, 3, 4, 5, 6]) {
            panic!("selection_sort-small_test2: {:?}", arr);
        }

        arr = orig.clone();
        insertion_sort(&mut arr);
        if !matches!(arr[..], [1, 2, 3, 4, 5, 6]) {
            panic!("insertion_sort-small_test2: {:?}", arr);
        }

        if !matches!(merge_sort(&orig)[..], [1, 2, 3, 4, 5, 6]) {
            panic!("merge_sort-small_test2: {:?}", arr);
        }
    }

    #[test]
    fn empty_input() {
        let orig = vec![];
        let mut arr;

        arr = orig.clone();
        selection_sort(&mut arr);
        if !matches!(arr[..], []) {
            panic!("selection_sort-empty_input: {:?}", arr);
        }

        arr = orig.clone();
        insertion_sort(&mut arr);
        if !matches!(arr[..], []) {
            panic!("insertion_sort-empty_input: {:?}", arr);
        }

        if !matches!(merge_sort(&orig)[..], []) {
            panic!("merge_sort-already_sorted: {:?}", arr);
        }
    }
}
