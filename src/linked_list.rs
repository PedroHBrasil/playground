mod node;

/// Represents a single-linked list
pub struct LinkedList<'a, T> {
    // First node of the linked list
    head: &'a node::NodeSingle<'a, T>
}