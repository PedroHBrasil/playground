use std::{alloc::LayoutError, error::Error};


struct Array<T> {
    pointer: *mut T,
    length: usize,
}

impl<T> Array<T> 
where
    T: Copy
{
    /// Initializes the array by allocating memory and setting the initial values
    pub fn new(length: usize, init_value: T) -> Result<Self, LayoutError> {
        let pointer = Self::alloc_mem(length)?;
        unsafe { Self::assign_init_values(&pointer, length, init_value) };

        Ok(Self {pointer, length})
    }

    /// Allocates the memory and assigns the initial values needed for the array
    fn alloc_mem(length: usize) -> Result<*mut T, LayoutError> {
        // Allocates the memory
        let layout = std::alloc::Layout::array::<T>(length)?;
        let pointer = unsafe {std::alloc::alloc(layout) as *mut T};

        Ok(pointer)
    }

    /// Sets elements values to initialization value
    unsafe fn assign_init_values(pointer: &*mut T, length: usize, init_value: T) -> () {
        for i in 0..length {
            let cur_ptr = pointer.offset(i as isize);
            *cur_ptr = init_value;
        }
    }

    // Gets an element
    pub fn get_at(&self, i: usize) -> Result<T, Box<dyn Error>> {
        // Makes sure index is in range
        if i >= self.length {
            return Err("Index out of bounds.".into());
        }
        // Gets ith element
        let value = unsafe {*self.pointer.offset(i as isize)};

        Ok(value)
    }
}

#[cfg(test)]
mod test{
    use std::error::Error;

    use super::*;

    #[test]
    fn init_array() -> Result<(), Box<dyn Error>> {
        let length = 3;
        let init_value = 1;
        let arr = Array::new(length, init_value)?;

        assert_eq!(arr.length, length);

        for i in 0..length {
            unsafe { assert_eq!(*arr.pointer.offset(i as isize), init_value) }
        }

        Ok(())
    }

    #[test]
    fn get() -> Result<(), Box<dyn Error>> {
        let length = 3;
        let init_value = 1;
        let arr = Array::new(length, init_value)?;

        for i in 0..length {
            assert_eq!(arr.get_at(i)?, init_value);
        }

        Ok(())
    }

    #[test]
    fn get_err() -> Result<(), Box<dyn Error>> {
        let length = 3;
        let init_value = 1;
        let arr = Array::new(length, init_value)?;

        assert!(arr.get_at(length).is_err());

        Ok(())
    }


}