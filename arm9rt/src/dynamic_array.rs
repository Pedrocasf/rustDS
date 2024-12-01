use crate::alloc::alloc::{alloc, dealloc, Layout, realloc};
use core::default::Default;
#[derive(Clone)]
pub struct DynamicArray<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> DynamicArray<T> {
    pub fn new(len: usize) -> Self {
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(len, core::mem::size_of::<T>());
            alloc(layout) as *mut T
        };
        Self { ptr, len }
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            unsafe { Some(&*(self.ptr.add(idx))) }
        } else {
            None
        }
    }
    pub fn get_mut(&self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            unsafe { Some(&mut *(self.ptr.add(idx))) }
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
}
impl<T> Default for DynamicArray<T>{
    fn default()-> Self{
        DynamicArray::new(1)
    }
}

impl<T> Drop for DynamicArray<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align_unchecked(self.len, core::mem::size_of::<T>()),
            )
        };
    }
}

impl<T> core::ops::Index<usize> for DynamicArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<T> core::ops::IndexMut<usize> for DynamicArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len{
            let new_size = if self.len * 2 > index {self.len *2} else{index+1};
            unsafe{
                let layout = Layout::from_size_align_unchecked(new_size, core::mem::size_of::<T>());
                realloc(self.ptr as *mut u8, layout, new_size);
            }
        }
        self.get_mut(index).unwrap()
    }
}