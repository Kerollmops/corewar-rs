pub trait ConstMemSize {
    const MEM_SIZE: usize;
}

pub trait MemSize {
    fn mem_size(&self) -> usize;
}

impl<T: ConstMemSize> MemSize for T {
    fn mem_size(&self) -> usize {
        <Self as ConstMemSize>::MEM_SIZE
    }
}
