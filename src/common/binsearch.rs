/// 二分搜索
fn bin_search<T: PartialOrd>(n: &[T], key: &T) -> i32 {
    let mut l = 0;
    let mut r = n.len() - 1;
    while l < r {
        let mid = l + (r - l) / 2;
        if &n[mid] < key {
            l = mid + 1;
        } else if &n[mid] > key {
            r = mid - 1;
        } else {
            return mid as i32;
        }
    };
    return -1;
}

#[test]
fn f1() {
    let vec1 = vec![1, 2, 3, 4, 11, 14, 15];
    assert_eq!(bin_search(&vec1,&4),3);
}
