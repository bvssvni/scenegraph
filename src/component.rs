//!  Components represent a generic Component to be added to a Node

use node::Node;

/// Represents a generic Component to be added to a Node
pub trait Component {
    /// Handles a message. Its type is identified by a ID string, 'msg_id'.
    /// Data is Some(T) or None.
    /// Each message has its type of data (or none at all).
    /// Should return true if the message could be handled or false if otherwise.
    fn handle_message<'a, T>(&mut self, msg_id: &'a str, data: Option<&'a T>) -> bool;
    /// Updates this component, given the delta time in seconds
    /// since the last parent Node update and the parent Node itself.
    /// Should return false on failure.
    fn update<'a>(&mut self, delta_seconds: f64, parent_node: &'a Node) -> bool;
}
