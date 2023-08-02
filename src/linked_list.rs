use std::{error::Error, rc::Rc, cell::RefCell};
use self::node::NodeSingle;

mod node;

/// Represents a single-linked list
pub struct LinkedList<T: Copy> {
    /// First node of the linked list
    head: Option<Rc<RefCell<NodeSingle<T>>>>,
    /// Number of nodes in linked list
    length: usize,
}

impl<T> LinkedList<T>
where
    T: Copy,
{
    /// CREATE

    /// Constructor
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    /// Inserts new data (at the end if no index is specified)
    pub fn insert(&mut self, new_data: T, i: Option<usize>) -> Result<(), Box<dyn Error>> {
        // Returns an error if i is out of range
        if let Some(i) = i {
            // i as Some needs to decide how to insert the data
            if i > self.length {  // invalid i
                return Err("Index out of range.".into())
            } else if i == 0 {  // new head
                self.push(new_data)?;
            } else if i == self.length {  // new tail
                self.append(new_data)?;
            } else {  // new common node
                self.insert_at(new_data, i)?;
            }
        } else {
            // i as None means that the data should be on a new tail
            if self.length == 0 {
                self.push(new_data)?;
            } else {
                self.append(new_data)?;
            }
        }

        Ok(())
    }

    // Inserts a new head for the linked list
    fn push(&mut self, new_data: T) -> Result<(), Box<dyn Error>> {
        if let Some(_cur_head) = self.head.clone() {
            let new_head = NodeSingle::new(new_data, self.head.clone());
            self.head = Some(Rc::new(RefCell::new(new_head)));
        } else {
            let new_head = NodeSingle::new(new_data, None);
            self.head = Some(Rc::new(RefCell::new(new_head)));
        }
        self.length += 1;

        Ok(())
    }

    // Inserts a new head for the linked list
    fn append(&mut self, new_data: T) -> Result<(), Box<dyn Error>> {
        // Gets current tail
        let cur_tail = self.get_node_at(self.length-1)?;
        // Initializes new tail
        let new_tail = NodeSingle::new(new_data, None);
        // Sets new tail as current tail's next node
        cur_tail.unwrap().borrow_mut().set_next(Some(Rc::new(RefCell::new(new_tail))));

        self.length += 1;

        Ok(())
    }

    fn insert_at(&mut self, new_data: T, i: usize) -> Result<(), Box<dyn Error>> {
        // Gets current i node
        let cur_node_at_i = self.get_node_at(i)?;
        // Sets current i node as next of new node
        let new_node = NodeSingle::new(new_data, cur_node_at_i);
        // Sets previous node's next node to new node
        (self.get_node_at(i-1)?).unwrap().borrow_mut().set_next(Some(Rc::new(RefCell::new(new_node))));

        self.length += 1;

        Ok(())
    }

    /// READ

    /// Returns a mutable reference to a node at the specified index
    fn get_node_at(&self, i: usize) -> Result<Option<Rc<RefCell<NodeSingle<T>>>>, Box<dyn Error>> {
        // Checks if i is valid
        if i > self.length {
            return Err("Index out of range.".into())
        }
        // Gets node by looping through list until counter is i or a None is found as next node
        let mut cur_node = self.head.clone();
        let mut counter = 0;
        loop {
            if let Some(node) = cur_node.clone() {
                if counter == i {
                    break;
                }
                cur_node = node.borrow().get_next();
                counter += 1;
            } else {
                return Err("Current node is None".into())
            }
        }

        Ok(cur_node)
    }

    /// Reads data at specified index
    pub fn read_at(&self, i: usize) -> Result<T, Box<dyn Error>> {
        let data = *self.get_node_at(i)?.unwrap().borrow().get_data();

        Ok(data)
    }
    /// Gets index of specified data in array
    pub fn index_of(&self, data: T) -> Option<usize> {
        None
    }

    /// UPDATE

    /// Updates data at specified node
    pub fn update_at(&self, new_data: T, i: usize) -> Result<(), Box<dyn Error>> {
        Err("Not implemented".into())
    }
    /// Swaps data of the specified nodes
    pub fn swap_at(&self, i_ref: usize, i_tgt: usize) -> Result<(), Box<dyn Error>> {
        Err("Not implemented".into())
    }

    /// DELETE

    /// Removes node at specified index
    pub fn remove_at(&self, i: usize) -> Result<(), Box<dyn Error>> {
        Err("Not implemented".into())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    // NEW
    #[test]
    fn new_linked_list() {
        // Data
        let linked_list: LinkedList<usize> = LinkedList::new();
        // Run
        let result_head = linked_list.head.clone();
        let result_length = linked_list.length;
        // Assert
        assert!(result_head.is_none());
        assert_eq!(result_length, 0);
    }

    // INSERT

    #[test]
    fn linked_list_empty_insert() {
        // Data
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        let _ = linked_list.insert(data, None);
        // Run
        let result = linked_list.head.clone();
        // Assert
        assert_eq!(result.unwrap().borrow().get_data(), &data);
    }

    #[test]
    fn linked_list_insert() {
        // Data
        let data0 = 0;
        let data1 = 1;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        // Run
        let _ = linked_list.insert(data0, None);
        let _ = linked_list.insert(data1, None);
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
        let _ = linked_list.insert(data0, None);
        let _ = linked_list.insert(data2, None);
        let _ = linked_list.insert(data1, Some(1));
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
    fn linked_list_get_node_at() {
        // Data
        let i = 0;
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        let _ = linked_list.insert(data, None);
        // Run
        let expected = linked_list.head.clone();
        let result = linked_list.get_node_at(i).unwrap();
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn linked_list_read_at_0() {
        // Data
        let i = 0;
        let data = 0;
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        let _ = linked_list.insert(data, None);
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
        let _ = linked_list.insert(data0, None);
        let _ = linked_list.insert(data1, None);
        let _ = linked_list.insert(data2, None);
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
        let _ = linked_list.insert(data, None);
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
        let _ = linked_list.insert(data, None);
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
        let _ = linked_list.insert(data0, None);
        let _ = linked_list.insert(data1, None);
        let _ = linked_list.insert(data2, None);
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
        let _ = linked_list.insert(data, None);
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
        let _ = linked_list.insert(data, None);
        let new_data = 1;
        // Run
        let _ = linked_list.update_at(new_data, i);
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
        let _ = linked_list.insert(data0, None);
        let _ = linked_list.insert(data1, None);
        let _ = linked_list.insert(data2, None);
        let new_data = 3;
        // Run
        let _ = linked_list.update_at(new_data, i);
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
        let _ = linked_list.insert(data, None);
        let new_data = 3;
        // Run
        let _ = linked_list.update_at(new_data, i);
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
        let _ = linked_list.insert(data_ref, None);
        let _ = linked_list.insert(data_tgt, None);
        // Run
        let _ = linked_list.swap_at(i_ref, i_tgt);
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
        let _ = linked_list.insert(data_ref, None);
        let _ = linked_list.insert(data_tgt, None);
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
        let _ = linked_list.insert(data0, None);
        let _ = linked_list.insert(data1, None);
        // Run
        let _ = linked_list.remove_at(i);
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
        let _ = linked_list.insert(data0, None);
        let _ = linked_list.insert(data1, None);
        let _ = linked_list.insert(data2, None);
        // Run
        let _ = linked_list.remove_at(i);
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
        let _ = linked_list.insert(data, None);
        // Run
        let result = linked_list.remove_at(i);
        // Assert
        assert!(result.is_err());
    }
}