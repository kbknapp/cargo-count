pub trait Comment {
    type Rep;
    fn single(&self) -> Option<Vec<<Self as Comment>::Rep>>;
    fn multi_start(&self) -> Option<<Self as Comment>::Rep>;
    fn multi_end(&self) -> Option<<Self as Comment>::Rep>;
}