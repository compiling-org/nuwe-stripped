use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_tiles::{Container, Tile, TileId, Tiles, UiResponse};
use std::collections::HashMap;
use crate::core::{NodeId, NodeGraph, VjEvent};
use super::{NodeType, NodeInstance};

#[derive(Resource)]
pub struct NodeGraphUI {
    pub tree: egui_tiles::Tree<NodePane>,
    pub selected_nodes: Vec<NodeId>,
    pub connection_start: Option<(NodeId, usize)>, // (node_id, output_index)
    pub show_grid: bool,
    pub grid_size: f32,
    pub zoom: f32,
    pub pan_offset: egui::Vec2,
}

impl Default for NodeGraphUI {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct NodePane {
    pub node_id: NodeId,
    pub node_type: NodeType,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
    pub title: String,
}

impl NodeGraphUI {
    pub fn new() -> Self {
        let mut ui = Self {
            tree: egui_tiles::Tree::empty("node_graph_tree"),
            selected_nodes: Vec::new(),
            connection_start: None,
            show_grid: true,
            grid_size: 50.0,
            zoom: 1.0,
            pan_offset: egui::Vec2::ZERO,
        };
        
        // Add some default nodes for demo
        ui.add_demo_nodes();
        ui
    }
    
    fn add_demo_nodes(&mut self) {
        let audio_gen = NodePane {
            node_id: NodeId::new(),
            node_type: NodeType::AudioGenerator,
            position: egui::Pos2::new(100.0, 100.0),
            size: egui::Vec2::new(150.0, 100.0),
            title: "Audio Generator".to_string(),
        };
        
        let visual_effect = NodePane {
            node_id: NodeId::new(),
            node_type: NodeType::VisualEffect,
            position: egui::Pos2::new(300.0, 150.0),
            size: egui::Vec2::new(150.0, 120.0),
            title: "Visual Effect".to_string(),
        };
        
        let output = NodePane {
            node_id: NodeId::new(),
            node_type: NodeType::Output,
            position: egui::Pos2::new(500.0, 125.0),
            size: egui::Vec2::new(120.0, 80.0),
            title: "Output".to_string(),
        };
        
        self.tree.tiles.insert_pane(audio_gen);
        self.tree.tiles.insert_pane(visual_effect);
        self.tree.tiles.insert_pane(output);
    }
}

pub struct NodeGraphUIPlugin;

impl Plugin for NodeGraphUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NodeGraphUI>()
            .add_systems(Update, render_node_graph_ui);
        
        info!("🎛️ Node graph UI plugin initialized");
    }
}

fn render_node_graph_ui(
    mut contexts: EguiContexts,
    mut node_ui: ResMut<NodeGraphUI>,
    node_graph: Res<NodeGraph>,
    mut event_writer: MessageWriter<VjEvent>,
) {
    let ctx = contexts.ctx_mut();
    
    egui::Window::new("Node Graph Editor")
        .default_size([800.0, 600.0])
        .show(ctx, |ui| {
            // Toolbar
            ui.horizontal(|ui| {
                if ui.button("🔴 Add Audio").clicked() {
                    // Add new audio node
                    let new_node = NodePane {
                        node_id: NodeId::new(),
                        node_type: NodeType::AudioGenerator,
                        position: egui::Pos2::new(200.0, 200.0),
                        size: egui::Vec2::new(150.0, 100.0),
                        title: "New Audio Generator".to_string(),
                    };
                    node_ui.tree.tiles.insert_pane(new_node);
                }
                
                if ui.button("🎨 Add Visual").clicked() {
                    let new_node = NodePane {
                        node_id: NodeId::new(),
                        node_type: NodeType::VisualEffect,
                        position: egui::Pos2::new(200.0, 300.0),
                        size: egui::Vec2::new(150.0, 120.0),
                        title: "New Visual Effect".to_string(),
                    };
                    node_ui.tree.tiles.insert_pane(new_node);
                }
                
                if ui.button("🔊 Add Output").clicked() {
                    let new_node = NodePane {
                        node_id: NodeId::new(),
                        node_type: NodeType::Output,
                        position: egui::Pos2::new(400.0, 250.0),
                        size: egui::Vec2::new(120.0, 80.0),
                        title: "New Output".to_string(),
                    };
                    node_ui.tree.tiles.insert_pane(new_node);
                }
                
                ui.separator();
                
                ui.checkbox(&mut node_ui.show_grid, "Grid");
                ui.add(egui::Slider::new(&mut node_ui.grid_size, 20.0..=100.0).text("Grid Size"));
                ui.add(egui::Slider::new(&mut node_ui.zoom, 0.5..=2.0).text("Zoom"));
            });
            
            ui.separator();
            
            // Main canvas area
            let canvas_rect = ui.available_rect_before_wrap();
            let canvas_response = ui.allocate_rect(canvas_rect, egui::Sense::click_and_drag());
            
            // Handle panning
            if canvas_response.dragged() {
                node_ui.pan_offset += canvas_response.drag_delta();
            }
            
            // Draw grid
            if node_ui.show_grid {
                draw_grid(ui, canvas_rect, &node_ui);
            }
            
            // Draw nodes
            for tile in node_ui.tree.tiles.tiles() {
                if let Tile::Pane(pane) = tile {
                    draw_node(ui, pane, canvas_rect, &node_ui);
                }
            }
            
            // Draw connections
            draw_connections(ui, &node_graph, &node_ui, canvas_rect);
        });
}

fn draw_grid(ui: &mut egui::Ui, rect: egui::Rect, node_ui: &NodeGraphUI) {
    let painter = ui.painter_at(rect);
    let grid_color = egui::Color32::from_gray(40);
    
    let grid_spacing = node_ui.grid_size * node_ui.zoom;
    let offset = node_ui.pan_offset;
    
    // Vertical lines
    let mut x = rect.min.x + (offset.x % grid_spacing);
    while x < rect.max.x {
        painter.line_segment(
            [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
            egui::Stroke::new(1.0, grid_color),
        );
        x += grid_spacing;
    }
    
    // Horizontal lines
    let mut y = rect.min.y + (offset.y % grid_spacing);
    while y < rect.max.y {
        painter.line_segment(
            [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
            egui::Stroke::new(1.0, grid_color),
        );
        y += grid_spacing;
    }
}

fn draw_node(ui: &mut egui::Ui, pane: &NodePane, canvas_rect: egui::Rect, node_ui: &NodeGraphUI) {
    let world_pos = pane.position * node_ui.zoom + node_ui.pan_offset;
    let world_size = pane.size * node_ui.zoom;
    
    // Check if node is visible
    let node_rect = egui::Rect::from_min_size(
        canvas_rect.min + world_pos.to_vec2(),
        world_size,
    );
    
    if !canvas_rect.intersects(node_rect) {
        return;
    }
    
    let painter = ui.painter_at(canvas_rect);
    
    // Node background
    let node_color = match pane.node_type {
        NodeType::AudioGenerator => egui::Color32::from_rgb(100, 150, 255),
        NodeType::VisualEffect => egui::Color32::from_rgb(255, 150, 100),
        NodeType::Output => egui::Color32::from_rgb(150, 255, 100),
        _ => egui::Color32::from_rgb(150, 150, 150),
    };
    
    let is_selected = false; // TODO: Check if node is selected
    let stroke_color = if is_selected {
        egui::Color32::WHITE
    } else {
        egui::Color32::GRAY
    };
    
    painter.rect_filled(
        node_rect,
        egui::Rounding::same(8),
        node_color,
    );
    
    painter.rect_stroke(
        node_rect,
        egui::Rounding::same(8),
        egui::Stroke::new(2.0, stroke_color),
    );
    
    // Node title
    let title_rect = egui::Rect::from_min_size(
        node_rect.min + egui::vec2(8.0, 4.0),
        egui::vec2(node_rect.width() - 16.0, 20.0),
    );
    
    painter.text(
        title_rect.center(),
        egui::Align2::CENTER_CENTER,
        &pane.title,
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );
    
    // Input/Output sockets
    draw_node_sockets(ui, pane, node_rect, &painter);
}

fn draw_node_sockets(ui: &egui::Ui, pane: &NodePane, node_rect: egui::Rect, painter: &egui::Painter) {
    let socket_radius = 6.0;
    let socket_color = egui::Color32::from_rgb(200, 200, 200);
    
    // Input sockets (left side)
    let input_count = match pane.node_type {
        NodeType::AudioGenerator => 0,
        NodeType::VisualEffect => 2,
        NodeType::Output => 1,
        _ => 1,
    };
    
    for i in 0..input_count {
        let socket_pos = egui::pos2(
            node_rect.min.x,
            node_rect.min.y + 30.0 + (i as f32 * 25.0),
        );
        
        painter.circle(
            socket_pos,
            socket_radius,
            socket_color,
            egui::Stroke::new(2.0, egui::Color32::BLACK),
        );
    }
    
    // Output sockets (right side)
    let output_count = match pane.node_type {
        NodeType::AudioGenerator => 1,
        NodeType::VisualEffect => 1,
        NodeType::Output => 0,
        _ => 1,
    };
    
    for i in 0..output_count {
        let socket_pos = egui::pos2(
            node_rect.max.x,
            node_rect.min.y + 30.0 + (i as f32 * 25.0),
        );
        
        painter.circle(
            socket_pos,
            socket_radius,
            socket_color,
            egui::Stroke::new(2.0, egui::Color32::BLACK),
        );
    }
}

fn draw_connections(ui: &egui::Ui, _node_graph: &NodeGraph, _node_ui: &NodeGraphUI, _canvas_rect: egui::Rect) {
    // TODO: Implement connection drawing when NodeGraph exposes public methods
    // For now, this is disabled to avoid accessing private fields
}

fn find_pane_by_node_id(tiles: &Tiles<NodePane>, node_id: NodeId) -> Option<&NodePane> {
    for tile in tiles.tiles() {
        if let Tile::Pane(pane) = tile {
            if pane.node_id == node_id {
                return Some(pane);
            }
        }
    }
    None
}

fn calculate_output_socket_pos(pane: &NodePane, node_ui: &NodeGraphUI, canvas_rect: egui::Rect, output_index: usize) -> egui::Pos2 {
    let world_pos = pane.position * node_ui.zoom + node_ui.pan_offset;
    let world_size = pane.size * node_ui.zoom;
    
    canvas_rect.min + egui::vec2(
        world_pos.x + world_size.x,
        world_pos.y + 30.0 + (output_index as f32 * 25.0),
    )
}

fn calculate_input_socket_pos(pane: &NodePane, node_ui: &NodeGraphUI, canvas_rect: egui::Rect, input_index: usize) -> egui::Pos2 {
    let world_pos = pane.position * node_ui.zoom + node_ui.pan_offset;
    
    canvas_rect.min + egui::vec2(
        world_pos.x,
        world_pos.y + 30.0 + (input_index as f32 * 25.0),
    )
}

fn draw_connection_curve(painter: &egui::Painter, start: egui::Pos2, end: egui::Pos2) {
    let control_offset = (end.x - start.x) * 0.5;
    let control1 = egui::pos2(start.x + control_offset, start.y);
    let control2 = egui::pos2(end.x - control_offset, end.y);
    
    let points: Vec<egui::Pos2> = (0..=50)
        .map(|i| {
            let t = i as f32 / 50.0;
            cubic_bezier(start, control1, control2, end, t)
        })
        .collect();
    
    painter.add(egui::Shape::line(
        points,
        egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 255, 100)),
    ));
}

fn cubic_bezier(p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, p3: egui::Pos2, t: f32) -> egui::Pos2 {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;
    
    egui::pos2(
        uuu * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + ttt * p3.x,
        uuu * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + ttt * p3.y,
    )
}