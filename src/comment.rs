/// Defines comment styles for any language
pub trait Comment {
    /// The type of the comment (`String`, `&'static str`, etc.)
    type Rep;
    /// Returns the single line comment style, if any
    fn single(&self) -> Option<Vec<<Self as Comment>::Rep>>;
    /// Returns the start of a multi-line comment style, if any
    fn multi_start(&self) -> Option<<Self as Comment>::Rep>;
    /// Returns the end of a multi-line comment style, if any
    fn multi_end(&self) -> Option<<Self as Comment>::Rep>;
}
