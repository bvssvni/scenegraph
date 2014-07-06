//! Scene represents the scene graph

use node::Node;

/// Handles the scene graph, and holds Nodes
pub trait Scene {
    /// Returns the Scene's root Node
    fn get_root_node(&mut self) -> &mut Node;
    /// Updates the scene. 'delta_seconds' is the
    /// delta time since the last update, in seconds.
    /// Returns false on failure.
    fn update(&mut self, delta_seconds: f64) -> bool;
}
