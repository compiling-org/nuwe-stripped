#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use crate::{NodeGraph, NodeId, VjEvent};

    #[test]
    fn test_node_creation() {
        let mut node_graph = NodeGraph::new();
        
        let node1 = node_graph.add_node();
        let node2 = node_graph.add_node();
        
        assert_eq!(node_graph.graph.node_count(), 2);
        assert_ne!(node1, node2);
    }

    #[test]
    fn test_node_connection() {
        let mut node_graph = NodeGraph::new();
        
        let node1 = node_graph.add_node();
        let node2 = node_graph.add_node();
        
        let result = node_graph.connect_nodes(node1, node2);
        assert!(result.is_ok());
        
        assert_eq!(node_graph.graph.edge_count(), 1);
    }

    #[test]
    fn test_invalid_node_connection() {
        let mut node_graph = NodeGraph::new();
        
        let node1 = node_graph.add_node();
        let invalid_node = NodeId::new();
        
        let result = node_graph.connect_nodes(node1, invalid_node);
        assert!(result.is_err());
        assert_eq!(node_graph.graph.edge_count(), 0);
    }

    #[test]
    fn test_node_removal() {
        let mut node_graph = NodeGraph::new();
        
        let node1 = node_graph.add_node();
        let node2 = node_graph.add_node();
        let node3 = node_graph.add_node();
        
        node_graph.connect_nodes(node1, node2).unwrap();
        node_graph.connect_nodes(node2, node3).unwrap();
        
        assert_eq!(node_graph.graph.node_count(), 3);
        assert_eq!(node_graph.graph.edge_count(), 2);
        
        node_graph.remove_node(node2);
        
        // Node should be removed along with its connections
        assert_eq!(node_graph.graph.node_count(), 2);
        assert_eq!(node_graph.graph.edge_count(), 0);
    }

    #[test]
    fn test_event_creation() {
        let event1 = VjEvent::NodeCreated {
            node_id: NodeId::new(),
            node_type: "generator".to_string(),
        };
        
        let event2 = VjEvent::NodeDestroyed {
            node_id: NodeId::new(),
        };
        
        // Just test that events can be created
        match event1 {
            VjEvent::NodeCreated { .. } => assert!(true),
            _ => assert!(false, "Wrong event type"),
        }
        
        match event2 {
            VjEvent::NodeDestroyed { .. } => assert!(true),
            _ => assert!(false, "Wrong event type"),
        }
    }

    #[test]
    fn test_topological_sort() {
        let mut node_graph = NodeGraph::new();
        
        let node1 = node_graph.add_node(); // Generator
        let node2 = node_graph.add_node(); // Effect 1
        let node3 = node_graph.add_node(); // Effect 2  
        let node4 = node_graph.add_node(); // Output
        
        // Create a chain: node1 -> node2 -> node3 -> node4
        node_graph.connect_nodes(node1, node2).unwrap();
        node_graph.connect_nodes(node2, node3).unwrap();
        node_graph.connect_nodes(node3, node4).unwrap();
        
        let sorted = node_graph.topological_sort();
        assert!(sorted.is_ok());
        
        let order = sorted.unwrap();
        assert_eq!(order.len(), 4);
        
        // Check that dependencies come before dependents
        let pos1 = order.iter().position(|&x| x == node1).unwrap();
        let pos2 = order.iter().position(|&x| x == node2).unwrap();
        let pos3 = order.iter().position(|&x| x == node3).unwrap();
        let pos4 = order.iter().position(|&x| x == node4).unwrap();
        
        assert!(pos1 < pos2);
        assert!(pos2 < pos3);
        assert!(pos3 < pos4);
    }

    #[test]
    fn test_cycle_detection() {
        let mut node_graph = NodeGraph::new();
        
        let node1 = node_graph.add_node();
        let node2 = node_graph.add_node();
        let node3 = node_graph.add_node();
        
        // Create a cycle: node1 -> node2 -> node3 -> node1
        node_graph.connect_nodes(node1, node2).unwrap();
        node_graph.connect_nodes(node2, node3).unwrap();
        node_graph.connect_nodes(node3, node1).unwrap();
        
        let sorted = node_graph.topological_sort();
        assert!(sorted.is_err()); // Should fail due to cycle
    }

    #[test]
    fn test_connection_queries() {
        let mut node_graph = NodeGraph::new();
        
        let node1 = node_graph.add_node();
        let node2 = node_graph.add_node();
        let node3 = node_graph.add_node();
        
        node_graph.connect_nodes(node1, node2).unwrap();
        node_graph.connect_nodes(node1, node3).unwrap();
        
        let outgoing = node_graph.get_connections_from(node1);
        assert_eq!(outgoing.len(), 2);
        assert!(outgoing.contains(&node2));
        assert!(outgoing.contains(&node3));
        
        let incoming_node2 = node_graph.get_connections_to(node2);
        assert_eq!(incoming_node2.len(), 1);
        assert!(incoming_node2.contains(&node1));
        
        let incoming_node1 = node_graph.get_connections_to(node1);
        assert_eq!(incoming_node1.len(), 0);
    }

    #[test]
    fn test_node_id_generation() {
        let mut node_graph = NodeGraph::new();
        
        let mut node_ids = Vec::new();
        for _ in 0..100 {
            node_ids.push(node_graph.add_node());
        }
        
        // All IDs should be unique
        for i in 0..node_ids.len() {
            for j in (i + 1)..node_ids.len() {
                assert_ne!(node_ids[i], node_ids[j]);
            }
        }
    }

    #[test]
    fn test_node_metadata() {
        // This would test node metadata functionality once implemented
        let node_id = NodeId::new();
        // Test that NodeId wraps a UUID
        assert_eq!(node_id.0, 42);
    }
}