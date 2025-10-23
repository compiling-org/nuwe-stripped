use bevy::prelude::*;
use petgraph::{Graph, Direction, Directed};
use petgraph::graph::{NodeIndex, EdgeIndex};
use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Deserialize, Serialize};
use crate::core::{NodeId, ConnectionId, DataType, VjEvent};

/// Node graph plugin
pub struct NodeGraphPlugin;

impl Plugin for NodeGraphPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NodeGraph>()
            .init_resource::<NodeRegistry>()
            .register_type::<NodePort>()
            .register_type::<NodeConnection>()
            .add_systems(Update, (
                evaluate_node_graph,
                propagate_dirty_nodes,
                update_node_positions,
            ).chain());
    }
}

/// System sets for organizing graph operations
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphSystemSet {
    DirtyPropagation,
    Evaluation,
    UI,
}

/// Main node graph resource using petgraph
#[derive(Resource, Debug)]
pub struct NodeGraph {
    graph: Graph<NodeId, ConnectionId, Directed>,
    node_to_index: HashMap<NodeId, NodeIndex>,
    index_to_node: HashMap<NodeIndex, NodeId>,
    connections: HashMap<ConnectionId, NodeConnection>,
    evaluation_order: Vec<NodeId>,
    dirty_nodes: HashSet<NodeId>,
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            node_to_index: HashMap::new(),
            index_to_node: HashMap::new(),
            connections: HashMap::new(),
            evaluation_order: Vec::new(),
            dirty_nodes: HashSet::new(),
        }
    }
}

impl NodeGraph {
    /// Add a node to the graph
    pub fn add_node(&mut self, node_id: NodeId) -> Result<(), GraphError> {
        if self.node_to_index.contains_key(&node_id) {
            return Err(GraphError::NodeAlreadyExists(node_id));
        }

        let index = self.graph.add_node(node_id);
        self.node_to_index.insert(node_id, index);
        self.index_to_node.insert(index, node_id);
        
        self.update_evaluation_order();
        self.mark_dirty(node_id);
        
        Ok(())
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, node_id: NodeId) -> Result<(), GraphError> {
        let index = self.node_to_index.remove(&node_id)
            .ok_or(GraphError::NodeNotFound(node_id))?;
        
        self.index_to_node.remove(&index);
        
        // Remove all connections involving this node
        let connections_to_remove: Vec<_> = self.connections
            .iter()
            .filter(|(_, conn)| conn.from_node == node_id || conn.to_node == node_id)
            .map(|(id, _)| *id)
            .collect();
            
        for conn_id in connections_to_remove {
            self.connections.remove(&conn_id);
        }
        
        self.graph.remove_node(index);
        self.dirty_nodes.remove(&node_id);
        self.update_evaluation_order();
        
        Ok(())
    }

    /// Add a connection between nodes
    pub fn add_connection(
        &mut self,
        from_node: NodeId,
        from_port: usize,
        to_node: NodeId,
        to_port: usize,
        data_type: DataType,
    ) -> Result<ConnectionId, GraphError> {
        let from_index = self.node_to_index.get(&from_node)
            .ok_or(GraphError::NodeNotFound(from_node))?;
        let to_index = self.node_to_index.get(&to_node)
            .ok_or(GraphError::NodeNotFound(to_node))?;

        // Check for cycles
        if self.would_create_cycle(*from_index, *to_index) {
            return Err(GraphError::CycleDetected);
        }

        let connection_id = ConnectionId::new();
        let connection = NodeConnection {
            id: connection_id,
            from_node,
            from_port,
            to_node,
            to_port,
            data_type,
        };

        self.graph.add_edge(*from_index, *to_index, connection_id);
        self.connections.insert(connection_id, connection);
        
        self.mark_dirty(to_node);
        self.update_evaluation_order();
        
        Ok(connection_id)
    }

    /// Remove a connection
    pub fn remove_connection(&mut self, connection_id: ConnectionId) -> Result<(), GraphError> {
        let connection = self.connections.remove(&connection_id)
            .ok_or(GraphError::ConnectionNotFound(connection_id))?;

        // Find and remove the edge
        let from_index = self.node_to_index[&connection.from_node];
        let to_index = self.node_to_index[&connection.to_node];
        
        if let Some(edge_index) = self.graph.find_edge(from_index, to_index) {
            self.graph.remove_edge(edge_index);
        }

        self.mark_dirty(connection.to_node);
        self.update_evaluation_order();
        
        Ok(())
    }

    /// Mark a node as dirty for re-evaluation
    pub fn mark_dirty(&mut self, node_id: NodeId) {
        self.dirty_nodes.insert(node_id);
        self.propagate_dirty_downstream(node_id);
    }

    /// Propagate dirty flag to downstream nodes
    fn propagate_dirty_downstream(&mut self, node_id: NodeId) {
        if let Some(&index) = self.node_to_index.get(&node_id) {
            let downstream_nodes: Vec<_> = self.graph
                .neighbors_directed(index, Direction::Outgoing)
                .filter_map(|idx| self.index_to_node.get(&idx))
                .copied()
                .collect();

            for downstream_node in downstream_nodes {
                if self.dirty_nodes.insert(downstream_node) {
                    // Recursively propagate if this node wasn't already dirty
                    self.propagate_dirty_downstream(downstream_node);
                }
            }
        }
    }

    /// Check if adding an edge would create a cycle
    fn would_create_cycle(&self, from_index: NodeIndex, to_index: NodeIndex) -> bool {
        // Use DFS to check if 'to' can reach 'from'
        let mut visited = HashSet::new();
        let mut stack = vec![to_index];

        while let Some(current) = stack.pop() {
            if current == from_index {
                return true;
            }
            
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            for neighbor in self.graph.neighbors_directed(current, Direction::Outgoing) {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        
        false
    }

    /// Update topological evaluation order using Kahn's algorithm
    fn update_evaluation_order(&mut self) {
        self.evaluation_order.clear();
        
        let mut in_degree: HashMap<NodeIndex, usize> = HashMap::new();
        let mut queue = VecDeque::new();

        // Calculate in-degrees
        for index in self.graph.node_indices() {
            let degree = self.graph.neighbors_directed(index, Direction::Incoming).count();
            in_degree.insert(index, degree);
            if degree == 0 {
                queue.push_back(index);
            }
        }

        // Kahn's algorithm
        while let Some(current) = queue.pop_front() {
            if let Some(&node_id) = self.index_to_node.get(&current) {
                self.evaluation_order.push(node_id);
            }

            for neighbor in self.graph.neighbors_directed(current, Direction::Outgoing) {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    /// Get evaluation order
    pub fn evaluation_order(&self) -> &[NodeId] {
        &self.evaluation_order
    }

    /// Get dirty nodes
    pub fn dirty_nodes(&self) -> &HashSet<NodeId> {
        &self.dirty_nodes
    }

    /// Clear dirty flag for a node
    pub fn clear_dirty(&mut self, node_id: NodeId) {
        self.dirty_nodes.remove(&node_id);
    }

    /// Get all connections for a node
    pub fn get_connections_for_node(&self, node_id: NodeId) -> Vec<&NodeConnection> {
        self.connections.values()
            .filter(|conn| conn.from_node == node_id || conn.to_node == node_id)
            .collect()
    }

    /// Get input connections for a node
    pub fn get_input_connections(&self, node_id: NodeId) -> Vec<&NodeConnection> {
        self.connections.values()
            .filter(|conn| conn.to_node == node_id)
            .collect()
    }

    /// Get output connections for a node
    pub fn get_output_connections(&self, node_id: NodeId) -> Vec<&NodeConnection> {
        self.connections.values()
            .filter(|conn| conn.from_node == node_id)
            .collect()
    }
}

/// Node connection data
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct NodeConnection {
    pub id: ConnectionId,
    pub from_node: NodeId,
    pub from_port: usize,
    pub to_node: NodeId,
    pub to_port: usize,
    pub data_type: DataType,
}

/// Node port definition
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct NodePort {
    pub name: String,
    pub port_type: PortType,
    pub data_type: DataType,
    pub required: bool,
}

/// Port type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Reflect, PartialEq)]
pub enum PortType {
    Input,
    Output,
}

/// Registry for available node types
#[derive(Resource, Debug, Default)]
pub struct NodeRegistry {
    pub node_types: HashMap<String, NodeTypeDefinition>,
}

impl NodeRegistry {
    pub fn register_node_type(&mut self, definition: NodeTypeDefinition) {
        self.node_types.insert(definition.name.clone(), definition);
    }
}

/// Definition of a node type
#[derive(Debug, Clone)]
pub struct NodeTypeDefinition {
    pub name: String,
    pub category: String,
    pub description: String,
    pub input_ports: Vec<NodePort>,
    pub output_ports: Vec<NodePort>,
    pub default_size: Vec2,
}

/// Graph-related errors
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Node {0:?} not found")]
    NodeNotFound(NodeId),
    #[error("Node {0:?} already exists")]
    NodeAlreadyExists(NodeId),
    #[error("Connection {0:?} not found")]
    ConnectionNotFound(ConnectionId),
    #[error("Adding connection would create a cycle")]
    CycleDetected,
    #[error("Port type mismatch")]
    PortTypeMismatch,
    #[error("Invalid port index")]
    InvalidPortIndex,
}

/// System to evaluate the node graph
fn evaluate_node_graph(
    graph: ResMut<NodeGraph>,
    mut _vj_events: MessageWriter<VjEvent>,
) {
    // Evaluate nodes in topological order
    for &node_id in graph.evaluation_order().clone().iter() {
        if graph.dirty_nodes().contains(&node_id) {
            // Node evaluation happens in individual node systems
            // This system just manages the overall flow
            debug!("Evaluating node: {:?}", node_id);
        }
    }
}

/// System to propagate dirty flags
fn propagate_dirty_nodes(_graph: ResMut<NodeGraph>) {
    // Dirty propagation is already handled in the mark_dirty method
    // This system could be used for additional propagation logic
}

/// System to update node positions (placeholder for UI integration)
fn update_node_positions() {
    // This will be implemented when we add the UI system
}