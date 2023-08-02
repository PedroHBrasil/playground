use std::{rc::Rc, cell::RefCell};


/// Represents a node of a single linked list
#[derive(PartialEq, Debug)]
pub struct NodeSingle<T: Copy> {
    /// Node's data
    data: T,
    /// Next node
    next: Option<Rc<RefCell<NodeSingle<T>>>>
}

impl<T> NodeSingle<T>
where
    T: Copy,
{
    pub fn new(data: T, next: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            data,
            next,
        }
    }
    /// Gets the node's data
    pub fn get_data(&self) -> &T {
        &self.data
    }
    /// Sets the node's data
    pub fn set_data(&mut self, new_data: T) -> () {
        self.data = new_data;
    }
    /// Gets the next node
    pub fn get_next(&self) -> Option<Rc<RefCell<Self>>> {
        self.next.clone()
    }
    /// Sets the next node
    pub fn set_next(&mut self, new_next: Option<Rc<RefCell<Self>>>) -> () {
        self.next = new_next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_data_node_single_i32() {
        // Data
        let data = 0;
        let next = None;
        let node = NodeSingle::new(data, next);
        // Run
        let result = *node.get_data();
        // Assert
        assert_eq!(result, data);
    }

    #[test]
    fn get_data_node_single_char() {
        // Data
        let data = 'a';
        let next = None;
        let node = NodeSingle::new(data, next);
        // Run
        let result = *node.get_data();
        // Assert
        assert_eq!(result, data);
    }

    #[test]
    fn set_data_node_single_i32() {
        // Data
        let data = 0;
        let next = None;
        let mut node = NodeSingle::new(data, next);
        let new_data = 1;
        // Run
        node.set_data(new_data);
        let result = *node.get_data();
        // Assert
        assert_eq!(result, new_data);
    }

    #[test]
    fn set_data_node_single_char() {
        // Data
        let data = 'a';
        let next = None;
        let mut node = NodeSingle::new(data, next);
        let new_data = 'b';
        // Run
        node.set_data(new_data);
        let result = *node.get_data();
        // Assert
        assert_eq!(result, new_data);
    }

    #[test]
    fn get_next_node_single_none() {
        // Data
        let data = 0;
        let next = None;
        let node = NodeSingle::new(data, next);
        // Run
        let result_node = node.get_next();
        let result = result_node;
        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_next_node_single_i32() {
        // Data
        let data1 = 1;
        let data0 = 0;
        let next1 = None;
        let node1 = Some(Rc::new(RefCell::new(NodeSingle::new(data1, next1))));
        let next0 = node1.clone();
        let node0 = NodeSingle::new(data0, next0);
        // Run
        let result = node0.get_next();
        // Assert
        assert_eq!(result, node1);
    }

    #[test]
    fn set_next_node_single_i32() {
        // Data
        let data0 = 0;
        let data1 = 1;
        let next1 = None;
        let node1 = Some(Rc::new(RefCell::new(NodeSingle::new(data1, next1))));
        let next0 = node1.clone();
        let mut node0 = NodeSingle::new(data0, next0);
        // Run
        let new_next0 = node1.clone();
        node0.set_next(new_next0);
        let result = node0.get_next();
        // Assert
        assert_eq!(result, node1);
    }

}