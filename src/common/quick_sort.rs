fn sort<T: PartialOrd>(arr: &mut [T]) {
    sort0(arr, 0, arr.len() as i32 - 1)
}

fn swap<T: PartialOrd>(arr: &mut [T], a: i32, b: i32) {
    arr.swap(a as usize, b as usize)
}

fn sort0<T: PartialOrd>(arr: &mut [T], start: i32, end: i32) {
    if start >= end { return; }
    let mut l = start;
    let mut r = end;
    let base = l;
    while l < r {
        while l < r && arr[base as usize] <= arr[r as usize] { r -= 1; }
        while l < r && arr[base as usize] >= arr[l as usize] { l += 1; }
        swap(arr, l, r);
    }
    swap(arr, l, start);
    sort0(arr, start, l - 1);
    sort0(arr, l + 1, end);
}

#[test]
fn f1() {
    let mut v = vec![4, 2, 1, 5, 6, 3, 11, 12];
    sort(&mut v);
    dbg!(v);
}
