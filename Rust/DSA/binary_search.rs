fn binary_search(nums: &[i32], n: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] == n {
            return Some(m);
        } else if nums[m] < n {
            l = m + 1;
        } else {
            r = m;
        }
    }
    None
}

fn main() {
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let index = binary_search(&nums, 9);
    match index {
            Some(idx) => println!("index: {} | numero: {}", idx, nums[idx]),
            None => println!("Número não encontrado no array."),
    }
}