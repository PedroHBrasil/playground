use std::error::Error;
use self::node::NodeSingle;

mod node;

/// Represents a single-linked list
pub struct LinkedList<'a, T> {
    /// First node of the linked list
    head: Option<&'a NodeSingle<'a, T>>,
    /// Number of nodes in linked list
    length: usize,
}

impl<'a, T> LinkedList<'a, T> {
    /// CREATE

    /// Constructor
    pub fn new() -> Self {}
    /// Inserts new data (at the end if no index is specified)
    pub fn insert(&self, new_data: T, i: Option<usize>) -> Result<(), Box<dyn Error>> {}

    /// READ

    /// Reads data at specified index
    pub fn read_at(&self, i: usize) -> Result<T, Box<dyn Error>> {}
    /// Gets index of specified data in array
    pub fn index_of(&self, data: T) -> Option<usize> {}

    /// UPDATE

    /// Updates data at specified node
    pub fn update_at(&self, new_data: T, i: usize) -> Result<(), Box<dyn Error>> {}
    /// Swaps data of the specified nodes
    pub fn swap_at(&self, i_ref: usize, i_tgt: usize) -> Result<(), Box<dyn Error>> {}

    /// DELETE

    /// Removes node at specified index
    pub fn remove_at(&self, i: usize) -> Result<(), Box<dyn Error>> {}

}


#[cfg(test)]
mod tests {
    use super::*;

    // NEW
    #[test]
    fn new_linked_list() {
        // Data
        //
        // Run
        let result: LinkedList<usize> = LinkedList::new();
        // Assert
        assert_eq!(result.head, None);
        assert_eq!(result.length, 0);
    }

    // INSERT

    #[test]
    fn linked_list_empty_insert() {
        // Data
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        // Run
        linked_list.insert(data, None);
        let result = *linked_list.head.unwrap().get_data();
        // Assert
        assert_eq!(result, data);
    }

    #[test]
    fn linked_list_insert() {
        // Data
        let data0 = 0;
        let data1 = 1;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        // Run
        linked_list.insert(data0, None);
        linked_list.insert(data1, None);
        let result0 = linked_list.read_at(0).unwrap();
        let result1 = linked_list.read_at(1).unwrap();
        // Assert
        assert_eq!(result0, data0);
        assert_eq!(result1, data1);
    }

    #[test]
    fn linked_list_insert_between() {
        // Data
        let data0 = 0;
        let data1 = 1;
        let data2 = 2;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        // Run
        linked_list.insert(data0, None);
        linked_list.insert(data2, None);
        linked_list.insert(data1, Some(1));
        let result0 = linked_list.read_at(0).unwrap();
        let result1 = linked_list.read_at(1).unwrap();
        let result2 = linked_list.read_at(2).unwrap();
        // Assert
        assert_eq!(result0, data0);
        assert_eq!(result1, data1);
        assert_eq!(result2, data2);
    }

    #[test]
    fn linked_list_insert_out_of_range() {
        // Data
        let i = 2;
        let data2 = 2;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        // Run
        let result = linked_list.insert(data2, Some(i));
        // Assert
        assert!(result.is_err());
    }

    // READ

    #[test]
    fn linked_list_read_at_0() {
        // Data
        let i = 1;
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data, None);
        // Run
        let result = linked_list.read_at(i).unwrap();
        // Assert
        assert_eq!(result, data);
    }

    #[test]
    fn linked_list_read_at_1() {
        // Data
        let i = 1;
        let data0 = 0;
        let data1 = 1;
        let data2 = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data0, None);
        linked_list.insert(data1, None);
        linked_list.insert(data2, None);
        // Run
        let result = linked_list.read_at(i).unwrap();
        // Assert
        assert_eq!(result, data1);
    }

    #[test]
    fn linked_list_read_out_of_range() {
        // Data
        let i = 1;
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data, None);
        // Run
        let result = linked_list.read_at(i);
        // Assert
        assert!(result.is_err());
    }

    // SEARCH

    #[test]
    fn linked_list_index_of_0() {
        // Data
        let i = 0;
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data, None);
        // Run
        let result = linked_list.index_of(data).unwrap();
        // Assert
        assert_eq!(result, i);
    }

    #[test]
    fn linked_list_index_of_1() {
        // Data
        let i = 1;
        let data0 = 0;
        let data1 = 1;
        let data2 = 2;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data0, None);
        linked_list.insert(data1, None);
        linked_list.insert(data2, None);
        // Run
        let result = linked_list.index_of(data1).unwrap();
        // Assert
        assert_eq!(result, i);
    }

    #[test]
    fn linked_list_index_non_existent() {
        // Data
        let data = 0;
        let data_search = 1;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data, None);
        // Run
        let result = linked_list.index_of(data_search);
        // Assert
        assert_eq!(result, None);
    }

    // UPDATE

    #[test]
    fn linked_list_update_at_0() {
        // Data
        let i = 0;
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data, None);
        let new_data = 1;
        // Run
        linked_list.update_at(new_data, i);
        let result = linked_list.read_at(i).unwrap();
        // Assert
        assert_eq!(result, new_data);
    }

    #[test]
    fn linked_list_update_at_1() {
        // Data
        let i = 1;
        let data0 = 0;
        let data1 = 1;
        let data2 = 2;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data0, None);
        linked_list.insert(data1, None);
        linked_list.insert(data2, None);
        let new_data = 3;
        // Run
        linked_list.update_at(new_data, i);
        let result = linked_list.read_at(i).unwrap();
        // Assert
        assert_eq!(result, new_data);
    }

    #[test]
    fn linked_list_update_out_of_range() {
        // Data
        let i = 1;
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data, None);
        let new_data = 3;
        // Run
        linked_list.update_at(new_data, i);
        let result = linked_list.read_at(i);
        // Assert
        assert!(result.is_err());
    }

    // SWAP

    #[test]
    fn linked_list_swap_at_0_1() {
        // Data
        let i_ref = 0;
        let i_tgt = 1;
        let data_ref = 0;
        let data_tgt = 1;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data_ref, None);
        linked_list.insert(data_tgt, None);
        // Run
        linked_list.swap_at(i_ref, i_tgt);
        let result0 = linked_list.read_at(0).unwrap();
        let result1 = linked_list.read_at(1).unwrap();
        // Assert
        assert_eq!(result0, data_tgt);
        assert_eq!(result1, data_ref);
    }

    #[test]
    fn linked_list_swap_at_0_out_of_range() {
        // Data
        let i_ref = 0;
        let i_tgt = 2;
        let data_ref = 0;
        let data_tgt = 1;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data_ref, None);
        linked_list.insert(data_tgt, None);
        // Run
        let result = linked_list.swap_at(i_ref, i_tgt);
        // Assert
        assert!(result.is_err());
    }

    // REMOVE

    #[test]
    fn linked_list_remove_at_0() {
        // Data
        let i = 0;
        let data0 = 0;
        let data1 = 1;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data0, None);
        linked_list.insert(data1, None);
        // Run
        linked_list.remove_at(i);
        let result = linked_list.read_at(i).unwrap();
        // Assert
        assert_eq!(result, data1);
    }

    #[test]
    fn linked_list_remove_at_1() {
        // Data
        let i = 1;
        let data0 = 0;
        let data1 = 1;
        let data2 = 2;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data0, None);
        linked_list.insert(data1, None);
        linked_list.insert(data2, None);
        // Run
        linked_list.remove_at(i);
        let result = linked_list.read_at(i).unwrap();
        // Assert
        assert_eq!(result, data2);
    }

    #[test]
    fn linked_list_remove_at_out_of_range() {
        // Data
        let i = 1;
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.insert(data, None);
        // Run
        let result = linked_list.remove_at(i);
        // Assert
        assert!(result.is_err());
    }
}