pub fn push_if_not_contained<T: PartialEq>(v: &mut Vec<T>, value: T) {
    if !v.contains(&value) {
        v.push(value);
    }
}
