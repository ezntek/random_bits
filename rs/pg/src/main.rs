fn binary_search(arr: &Vec<i32>, target: i32) -> bool {
    let middle = arr.len() / 2;

    if target > arr[arr.len()-1] {
        return false;
    }

    use std::cmp::Ordering::*;
    match target.cmp(&arr[middle]) {
        Less => binary_search(&Vec::from(&arr[..middle+1]), target),
        Greater => binary_search(&Vec::from(&arr[middle..]), target),
        Equal => true
    }
}

fn main() {
    let a = vec![1,4,5,6,7,8,12,63,566];
    let result = binary_search(&a, 1325);

    assert_eq!(false, result); 
}
