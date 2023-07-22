use std::error::Error;


/// Represents a node of a single linked list
pub struct NodeSingle<'a, T> {
    /// Node's data
    data: T,
    /// Next node
    next: Option<&'a NodeSingle<'a, T>>
}

impl<'a, T> NodeSingle<'a, T> {
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
            Err("No next node for current node.".to_string().into())
        }
    }
    /// Sets the next node
    pub fn set_next(&mut self, new_next: Option<&'a NodeSingle<'a, T>>) -> () {
        self.next = new_next;
    }
}