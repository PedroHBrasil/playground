use std::alloc::LayoutError;


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

        unsafe {
            let elem0 = *arr.pointer;
            let elem1 = *arr.pointer;
            let elem2 = *arr.pointer;
            assert_eq!(elem0, init_value);
            assert_eq!(elem1, init_value);
            assert_eq!(elem2, init_value);
        }

        Ok(())
    }
}