use std::error::Error;
use self::node::NodeSingle;

mod node;

/// Represents a single-linked list
pub struct LinkedList<'a, T> {
    // First node of the linked list
    head: &'a NodeSingle<'a, T>
}

impl<'a, T> LinkedList<'a, T> {

    /// Gets value of first node
    pub fn get_first_value(&self) -> &T {
        self.head.get_data()
    }

    /// Gets value of last node
    pub fn get_last_value(&self) -> &T {
        // Sweeps nodes until the last one (.next is None) is found
        let mut cur_node = self.head;
        while let Ok(next_node) = cur_node.get_next() {
            cur_node = next_node;
        }

        cur_node.get_data()
    }

    /// Gets value of node at index i
    pub fn get_value_at(&self, i: u32) -> Result<&T, Box<dyn Error>> {
        // Sweeps nodes until the target index. Panics if the index is bigger than the number of nodes.
        let mut cur_node = self.head;
        let mut counter: u32 = 0;
        while counter != i {
            cur_node = cur_node.get_next()?;
            counter += 1;
        }

        Ok(cur_node.get_data())
    }

}