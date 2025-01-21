use std::{
    alloc::{self, alloc, realloc, Layout},
    fmt, mem,
    ops::{Index, IndexMut},
    ptr,
};

pub struct Vector<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            Self {
                ptr: ptr::null_mut(),
                len: 0,
                capacity: 0,
            }
        } else {
            //size calculation
            let size_t = mem::size_of::<T>();
            let needed_size = size_t.checked_mul(capacity).expect("Overflow");

            //allocate space
            let layout =
                Layout::from_size_align(needed_size, mem::align_of::<T>()).expect("Invalid Layout");
            let ptr = unsafe { alloc(layout) } as *mut T;

            Self {
                ptr,
                len: 0,
                capacity,
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.add(self.len), val);
        }

        self.len += 1;
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };
        let new_size = mem::size_of::<T>()
            .checked_mul(new_capacity)
            .expect("Overflow");

        let new_layout =
            Layout::from_size_align(new_size, mem::align_of::<T>()).expect("Invalid Layout");

        unsafe {
            let new_ptr = if self.capacity == 0 {
                alloc(new_layout) as *mut T
            } else {
                let old_size = mem::size_of::<T>() * self.capacity;
                let old_layout = Layout::from_size_align(old_size, mem::align_of::<T>())
                    .expect("Invalid reallocation layout");
                realloc(self.ptr as *mut u8, old_layout, new_size) as *mut T
            };

            if new_ptr.is_null() {
                alloc::handle_alloc_error(new_layout);
            }

            self.ptr = new_ptr;
        }

        self.capacity = new_capacity;
    }

    pub fn print(&self)
    where
        T: fmt::Display,
    {
        unsafe {
            // Create a slice from the raw pointer for safe iteration.
            let elements = std::slice::from_raw_parts(self.ptr, self.len);

            print!("[");
            for (i, elem) in elements.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", elem);
            }
            println!("]");
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;

            unsafe {
                let element_ptr = self.ptr.add(self.len);
                Some(ptr::read(element_ptr))
            }
        }
    }

    pub fn is_emepty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        unsafe {
            let slice = ptr::slice_from_raw_parts_mut(self.ptr, self.len);
            ptr::drop_in_place(slice);
        }
        self.len = 0;
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Vector: Index out of range");
        }
        unsafe { &*self.ptr.add(index) }
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Vector: Index out of range");
        }
        unsafe { &mut *self.ptr.add(index) }
    }
}
