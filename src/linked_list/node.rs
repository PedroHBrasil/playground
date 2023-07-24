use std::error::Error;


/// Represents a node of a single linked list
#[derive(PartialEq, Debug)]
pub struct NodeSingle<'a, T> {
    /// Node's data
    data: T,
    /// Next node
    next: Option<&'a NodeSingle<'a, T>>
}

impl<'a, T> NodeSingle<'a, T> {
    pub fn new(data: T, next: Option<&'a NodeSingle<'a, T>>) -> Self {
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
    pub fn get_next(&self) -> Result<&Self, Box<dyn Error>> {
        if let Some(next_node) = self.next {
            Ok(next_node)
        } else {
            Err("No next node for current node.".into())
        }
    }
    /// Sets the next node
    pub fn set_next(&self, new_next: Option<&'a NodeSingle<'a, T>>) -> () {
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
        let node = NodeSingle::new(data, None);
        // Run
        let result = *node.get_data();
        // Assert
        assert_eq!(result, data);
    }

    #[test]
    fn get_data_node_single_char() {
        // Data
        let data = 'a';
        let node = NodeSingle::new(data, None);
        // Run
        let result = *node.get_data();
        // Assert
        assert_eq!(result, data);
    }

    #[test]
    fn set_data_node_single_i32() {
        // Data
        let data = 0;
        let mut node = NodeSingle::new(data, None);
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
        let mut node = NodeSingle::new(data, None);
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
        let node = NodeSingle::new(data, None);
        // Run
        let result = node.get_next();
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn get_next_node_single_i32() {
        // Data
        let data1 = 1;
        let data0 = 0;
        let mut node1 = NodeSingle::new(data1, None);
        let node0 = NodeSingle::new(data0, Some(&mut node1));
        // Run
        let result = node0.get_next().unwrap();
        // Assert
        assert_eq!(*result, node1);
    }

    #[test]
    fn set_next_node_single_i32() {
        // Data
        let data0 = 0;
        let data1 = 1;
        let mut node0 = NodeSingle::new(data0, None);
        let mut node1 = NodeSingle::new(data1, None);
        // Run
        node0.set_next(Some(&mut node1));
        let result = node0.next.unwrap();
        // Assert
        assert_eq!(*result, node1);
    }

}