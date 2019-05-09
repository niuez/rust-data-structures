
use std::mem;
use std::ptr::{ NonNull, self };
use std::alloc::{ Layout, alloc, realloc, dealloc };


pub struct Vector<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Vector { ptr: NonNull::dangling(), len: 0, cap: 0 }
    }

    fn grow(&mut self) {
        unsafe {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();

            let (new_cap, ptr) = if self.cap == 0 {
                let ptr = alloc(Layout::from_size_align_unchecked(elem_size, align));
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_num_bytes = self.cap * elem_size;

                assert!(old_num_bytes <= (::std::isize::MAX as usize) / 2, "capacity overflow");

                let new_num_bytes = old_num_bytes * 2;
                let ptr = realloc(self.ptr.as_ptr() as *mut _, 
                                  Layout::from_size_align_unchecked(old_num_bytes, align),
                                  new_num_bytes);
                (new_cap, ptr)
            };

            if ptr.is_null() { assert!(false, "ptr is null"); }
            self.ptr = NonNull::new_unchecked(ptr as *mut _);
            self.cap = new_cap;
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap { self.grow() }

        unsafe {
            ptr::write(self.ptr.as_ptr().offset(self.len as isize), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.as_ptr().offset(self.len as isize)))
            }
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}

            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let num_bytes = elem_size * self.cap;

            unsafe {
                dealloc(self.ptr.as_ptr() as *mut _, Layout::from_size_align_unchecked(num_bytes, align));
            }
        }
    }
}

#[test]
fn vector_test() {
    let mut v = Vector::new();
    v.push(0);
    v.push(1);
    v.push(2);
    v.push(0);
    v.push(1);
    v.push(2);
    v.push(0);
    v.push(1);
    v.push(2);
    assert!(v.pop() == Some(2));
    assert!(v.pop() == Some(1));
    assert!(v.pop() == Some(0));
    assert!(v.pop() == Some(2));
    assert!(v.pop() == Some(1));
    assert!(v.pop() == Some(0));
    assert!(v.pop() == Some(2));
    assert!(v.pop() == Some(1));
    assert!(v.pop() == Some(0));
    assert!(v.pop() == None);
}
