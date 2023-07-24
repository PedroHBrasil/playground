use std::error::Error;
use self::node::NodeSingle;

mod node;

/// Represents a single-linked list
pub struct LinkedList<'a, T> {
    /// First node of the linked list
    head: Option<&'a NodeSingle<'a, T>>,
    /// Number of nodes in linked list
    n_nodes: usize,
}

impl<'a, T> LinkedList<'a, T> {
    /// CREATE

    /// Constructor
    pub fn new(head_data: Option<T>) -> Self {}
    /// Inserts new data (at the end if no index is specified)
    pub fn insert(new_data: T, i: Option<usize>) -> Result<(), Box<dyn Error>> {}

    /// READ

    /// Reads data at specified index
    pub fn data_at(i: usize) -> Result<T, Box<dyn Error>> {}
    /// Gets index of specified data in array
    pub fn index_of(data: T) -> Option<usize> {}

    /// UPDATE

    /// Updates data at specified node
    pub fn update_at(new_data: T, i: usize) -> Result<(), Box<dyn Error>> {}
    /// Swaps data of the specified nodes
    pub fn swap_at(i_ref: usize, i_tgt: usize) -> Result<(), Box<dyn Error>> {}

    /// DELETE

    /// Removes node at specified index
    pub fn remove_at(i: usize) -> Result<(), Box<dyn Error>> {}

}
