
/// Represents a node of a single linked list
pub struct NodeSingle<'a, T> {
    /// Node's data
    data: T,
    /// Next node
    next: Option<&'a NodeSingle<'a, T>>
}

impl<'a, T> NodeSingle<'a, T> {
    /// Gets the node's data
    fn get_data(&self) -> &T {
        &self.data
    }
    /// Sets the node's data
    fn set_data(&mut self, new_data: T) -> () {
        self.data = new_data;
    }
    /// Gets the next node
    fn get_next(&self) -> Option<&Self> {
        self.next
    }
    /// Sets the next node
    fn set_next(&mut self, new_next: Option<&'a NodeSingle<'a, T>>) -> () {
        self.next = new_next;
    }
}