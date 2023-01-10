pub struct PageResult<E> {
    pub data: Vec<E>,
    pub page: u64,
    pub size: u64,
}
