fn median(arr: &[i32]) -> Option<f64> {
    if arr.is_empty() {
        return None;
    }
    let mid = arr.len() / 2;
    if arr.len() % 2 == 0 {
        Some((arr[mid - 1] + arr[mid]) as f64 / 2.0)
    } else {
        Some(arr[mid] as f64)
    }
}
