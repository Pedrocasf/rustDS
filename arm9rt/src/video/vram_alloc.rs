use alloc::collections::LinkedList;
use core::alloc::Allocator;

pub struct LinkedListAllocator {
    head: ListNode
}
impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self { head:ListNode::new(0) }
    }
}
struct ListNode{
    base: usize,
    size: usize,
    next:Option<&'static mut ListNode>,
}
impl ListNode{
    const fn new(base:usize,size:usize)->Self{
        ListNode{base,size,next:None}
    }
    fn start_addr(&self)->usize{
        self.base as *const Self as usize
    }
    fn end_addr(&self)->usize{
        self.start_addr() + self.size
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize){
        self.add_free_region(heap_start, heap_size);
    }
    pub unsafe fn add_free_region(&mut self,start: usize, size: usize){

    }
}
impl Allocator for LinkedListAllocator {
}