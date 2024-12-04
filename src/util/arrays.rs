pub fn remove_element(arr: Vec<i64>, index: usize) -> (Option<i64>, Vec<i64>) {
    let mut arr = arr.clone();
    if index < arr.len() {
        let removed = arr.remove(index);
        (Some(removed), arr)
    } else {
        (None, arr)
    }
}
