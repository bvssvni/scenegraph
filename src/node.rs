//! Node is a node (vertex) in the scene graph

use component::Component;

/// Represents a node in the scene graph, that can hold Components
pub trait Node {
    /// Returns Some(node) if this Node has a parent node, else returns None
    fn get_parent_node<'a>(&mut self) -> Option<&'a mut Node>;
    /// Returns all of the children Nodes of this Node
    fn get_children_nodes<'a>(&mut self) -> &'a mut Vec<&mut Node>;
    /// Returns all of the children Components of this Node
    fn get_components<'a>(&mut self) -> &'a mut Vec<&mut Component>;
    /// Updates the current Node's components and its subnodes. Returns false on failure
    fn update<'a>(&mut self, delta_seconds: f64, root_node: &'a mut Node) -> bool;
}
