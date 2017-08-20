pub trait MemSize {
    /// The number of bytes this instruction takes.
    fn mem_size(&self) -> usize;
}
