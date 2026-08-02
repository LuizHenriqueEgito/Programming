use std::cmp::min;

fn binary_search(
    nums: &[i32],
    n: i32,
    mut l: usize,
    mut r: usize
) -> Option<usize> {
    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m] == n {
            return Some(m);
        } else if nums[m] < n {
            l = m + 1;
        } else {
            if m == 0 {
                break;
            }
            r = m - 1;
        }
    }
    None
}

fn exponential_search(nums: &[i32], n: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    if nums[0] == n {
        return Some(0);
    }
    let len = nums.len();
    let mut i = 1;
    while i < len && nums[i] < n {
        i *= 2;
    }
    if i < len && nums[i] == n {
        return Some(i);
    }
    let l = i / 2;
    let r = min(i, len - 1);
    binary_search(nums, n, l, r)  // esse é o retorno
}

fn main() {
    let nums = [1,2,3,4,5,6,7,8,9];
    let index = exponential_search(&nums, 9);
    match index {
        Some(idx) => println!("index: {} | numero: {}", idx, nums[idx]),
        None => println!("Número não encontrado no array."),
    }
}