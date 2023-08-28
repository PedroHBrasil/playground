use std::{alloc::LayoutError, error::Error};


struct Array<T> {
    pointer: *mut T,
    length: usize,
}

impl<T> Array<T> 
where
    T: Copy + std::cmp::PartialEq
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

    /// Gets the value of an element
    pub fn get_at(&self, i: usize) -> Result<&T, Box<dyn Error>> {
        // Makes sure index is in range
        if i >= self.length {
            return Err("Index out of bounds.".into());
        }
        // Gets ith element
        let value = unsafe {self.pointer.offset(i as isize).as_ref()};
        if let Some(value) = value {
            Ok(value)
        } else {
            Err("Null pointer found.".into())
        }
    }

    /// Sets the value of an element
    pub fn set_at(&self, i: usize, new_value: T) -> Result<(), Box<dyn Error>> {
        // Makes sure index is in range
        if i >= self.length {
            return Err("Index out of bounds.".into());
        }
        // Sets ith element
        let element = unsafe { self.pointer.offset(i as isize).as_mut() };
        if let Some(element) = element {
            *element = new_value;
        } else {
            return Err("Null pointer found".into())
        }

        Ok(())
    }

    /// Finds the index of a value
    pub fn index_of(&self, search_value: T) -> Result<usize, Box<dyn Error>> {
        for i in 0..self.length {
            let value = self.get_at(i)?;
            if value == &search_value {
                return Ok(i);
            }
        }

        Err("Value not found.".into())
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        // Deallocates memory
        let layout = std::alloc::Layout::array::<T>(self.length).unwrap();
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.pointer, self.length));
            std::alloc::dealloc(self.pointer as *mut u8, layout)
        };
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

        // Default case
        for i in 0..length {
            assert_eq!(arr.get_at(i)?, &init_value);
        }

        // Error case
        assert!(arr.get_at(length).is_err());

        Ok(())
    }

    #[test]
    fn set() -> Result<(), Box<dyn Error>> {
        let length = 3;
        let init_value = 1;
        let arr = Array::new(length, init_value)?;

        // Default case
        for i in 0..length {
            arr.set_at(i, i)?;
            assert_eq!(arr.get_at(i)?, &i);
        }

        // Error case
        assert!(arr.set_at(length, init_value).is_err());

        Ok(())
    }

    #[test]
    fn index_of() -> Result<(), Box<dyn Error>> {
        let length = 3;
        let init_value = 1;
        let arr = Array::new(length, init_value)?;

        // Must return first found (0)
        for _ in 0..length {
            assert_eq!(arr.index_of(init_value)?, 0);
        }

        // Default behavior
        for i in 0..length {
            arr.set_at(i, i*2)?;
            assert_eq!(arr.index_of(i*2)?, i);
        }

        // Error: value not found
        assert!(arr.index_of(init_value).is_err());

        Ok(())
    }

    fn drop() -> Result<(), Box<dyn Error>> {
        let length = 3;
        let init_value = 1;
        // Initializes the array and drops it by letting it go out of scope.
        let ptr: *mut i32;
        { 
            let arr = Array::new(length, init_value)?;
            ptr = arr.pointer;
        }
        // Checks if array pointers are null (array has been dropped if they are)
        for i in 0..length {
            unsafe { assert!(ptr.offset(i as isize).is_null()) }
        }

        Ok(())
    }

}