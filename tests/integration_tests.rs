use bevy::prelude::*;
use immersive_vj_system::*;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.build().disable::<bevy::winit::WinitPlugin>(),
            ImmersiveVjPlugin,
        ));
        
        // Run one frame to ensure everything initializes
        app.update();
        
        // Check that core resources exist
        assert!(app.world().contains_resource::<NodeGraph>());
    }

    #[test]
    fn test_mcp_server_initialization() {
        let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.build().disable::<bevy::winit::WinitPlugin>(),
            ImmersiveVjPlugin,
        ));
        
        app.update();
        
        // Check that MCP server registry exists
        assert!(app.world().contains_resource::<mcp_servers::McpServerRegistry>());
        
        let registry = app.world().resource::<mcp_servers::McpServerRegistry>();
        assert_eq!(registry.servers.len(), 3); // Rust, Bevy, Shader servers
    }

    #[test]
    fn test_visual_system_initialization() {
        let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.build().disable::<bevy::winit::WinitPlugin>(),
            ImmersiveVjPlugin,
        ));
        
        app.update();
        
        // Check visual system resources
        assert!(app.world().contains_resource::<visual::ShaderRegistry>());
        assert!(app.world().contains_resource::<visual::VisualEffectsState>());
    }

    #[test]
    fn test_node_creation_and_connection() {
        let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.build().disable::<bevy::winit::WinitPlugin>(),
            ImmersiveVjPlugin,
        ));
        
        app.update();
        
        let mut node_graph = app.world_mut().resource_mut::<NodeGraph>();
        
        // Create test nodes
        let audio_node = node_graph.add_node();
        let visual_node = node_graph.add_node();
        let output_node = node_graph.add_node();
        
        // Connect them
        node_graph.connect_nodes(audio_node, visual_node).unwrap();
        node_graph.connect_nodes(visual_node, output_node).unwrap();
        
        // Verify connections
        assert_eq!(node_graph.get_connections_from(audio_node).len(), 1);
        assert_eq!(node_graph.get_connections_to(output_node).len(), 1);
    }

    #[test]
    fn test_event_handling() {
        let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.build().disable::<bevy::winit::WinitPlugin>(),
            ImmersiveVjPlugin,
        ));
        
        app.update();
        
        // Send a test event
        let mut event_writer = app.world_mut().resource_mut::<Events<VjEvent>>();
        event_writer.send(VjEvent::NodeCreated {
            node_id: NodeId(999),
            node_type: "test".to_string(),
        });
        
        // Process the event
        app.update();
        
        // Event should be processed (no crash)
    }

    #[test]
    fn test_shader_compilation() {
        let basic_shader = r#"
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(uv, 0.5, 1.0);
}
"#;
        
        let result = visual::shader_loader::ShaderCompiler::compile_wgsl(
            basic_shader.to_string(), 
            "test_shader"
        );
        
        assert!(result.is_ok(), "Basic shader compilation should succeed");
    }

    #[test]
    fn test_invalid_shader_compilation() {
        let invalid_shader = "this is not valid WGSL code";
        
        let result = visual::shader_loader::ShaderCompiler::compile_wgsl(
            invalid_shader.to_string(),
            "invalid_shader"
        );
        
        assert!(result.is_err(), "Invalid shader should fail compilation");
    }

    #[test]
    fn test_effect_chain_creation() {
        let mut chain = visual::effects::EffectChain::new();
        
        let color_grade = visual::effects::EffectChain::create_color_grade_effect(
            "test_color_grade".to_string()
        );
        
        chain.add_effect(color_grade);
        assert_eq!(chain.effects.len(), 1);
        
        let blur = visual::effects::EffectChain::create_blur_effect(
            "test_blur".to_string(),
            2.0
        );
        
        chain.add_effect(blur);
        assert_eq!(chain.effects.len(), 2);
        
        // Test removing effect
        chain.remove_effect("test_color_grade");
        assert_eq!(chain.effects.len(), 1);
        assert_eq!(chain.effects[0].id, "test_blur");
    }

    #[test]
    fn test_mcp_server_capabilities() {
        use immersive_vj_system::mcp_servers::*;
        
        let rust_server = rust_server::RustLanguageServer::new();
        let bevy_server = bevy_server::BevyDevServer::new();
        let shader_server = shader_server::ShaderDevServer::new();
        
        // Test capability requests
        let cap_request = McpRequest {
            id: uuid::Uuid::new_v4(),
            method: "capabilities".to_string(),
            params: serde_json::Value::Null,
        };
        
        let rust_caps = rust_server.handle_request(cap_request.clone()).unwrap();
        let bevy_caps = bevy_server.handle_request(cap_request.clone()).unwrap();
        let shader_caps = shader_server.handle_request(cap_request).unwrap();
        
        // Each server should return capabilities
        assert!(rust_caps.result.is_object());
        assert!(bevy_caps.result.is_object());
        assert!(shader_caps.result.is_object());
    }
}