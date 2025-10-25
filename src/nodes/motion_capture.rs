//! Motion Capture Integration for NUWE
//!
//! This module provides gesture tracking and motion capture functionality
//! for the NUWE node-based system, supporting MediaPipe and LeapMotion.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Gesture data from motion capture systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureData {
    pub gesture_type: String,
    pub confidence: f32,
    pub parameters: Vec<f32>,
    pub timestamp: f64,
}

/// Motion capture processor for gesture recognition
pub struct MotionCaptureProcessor {
    gesture_history: Vec<GestureData>,
    max_history_size: usize,
}

impl MotionCaptureProcessor {
    pub fn new() -> Self {
        Self {
            gesture_history: Vec::new(),
            max_history_size: 1000,
        }
    }

    /// Process gesture data from MediaPipe
    pub fn process_mediapipe_data(
        &mut self,
        hand_landmarks: Option<&Value>,
        pose_landmarks: Option<&Value>,
    ) -> Result<GestureData, Box<dyn std::error::Error>> {
        let gesture_data = self.extract_gesture_from_mediapipe(hand_landmarks, pose_landmarks)?;
        self.gesture_history.push(gesture_data.clone());

        if self.gesture_history.len() > self.max_history_size {
            self.gesture_history.remove(0);
        }

        Ok(gesture_data)
    }

    /// Process gesture data from LeapMotion
    pub fn process_leapmotion_data(
        &mut self,
        hand_positions: Option<&Value>,
        finger_positions: Option<&Value>,
        gestures: Option<&Value>,
    ) -> Result<GestureData, Box<dyn std::error::Error>> {
        let gesture_data = self.extract_gesture_from_leapmotion(hand_positions, finger_positions, gestures)?;
        self.gesture_history.push(gesture_data.clone());

        if self.gesture_history.len() > self.max_history_size {
            self.gesture_history.remove(0);
        }

        Ok(gesture_data)
    }

    /// Extract gesture data from MediaPipe landmarks
    fn extract_gesture_from_mediapipe(
        &self,
        hand_landmarks: Option<&Value>,
        pose_landmarks: Option<&Value>,
    ) -> Result<GestureData, Box<dyn std::error::Error>> {
        let gesture_type = if let Some(hand) = hand_landmarks {
            // Analyze hand landmarks for gesture
            "open_palm".to_string() // Placeholder - would use ML model in practice
        } else if let Some(pose) = pose_landmarks {
            // Analyze pose landmarks for gesture
            "standing".to_string() // Placeholder
        } else {
            "unknown".to_string()
        };

        Ok(GestureData {
            gesture_type,
            confidence: 0.8,
            parameters: vec![0.5, 0.3, 0.7], // Placeholder parameters
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        })
    }

    /// Extract gesture data from LeapMotion data
    fn extract_gesture_from_leapmotion(
        &self,
        hand_positions: Option<&Value>,
        finger_positions: Option<&Value>,
        gestures: Option<&Value>,
    ) -> Result<GestureData, Box<dyn std::error::Error>> {
        let gesture_type = if let Some(gesture) = gestures {
            if let Some(gesture_str) = gesture.as_str() {
                gesture_str.to_string()
            } else {
                "unknown".to_string()
            }
        } else {
            "open_hand".to_string() // Default
        };

        Ok(GestureData {
            gesture_type,
            confidence: 0.85,
            parameters: vec![0.4, 0.6, 0.2], // Placeholder parameters
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        })
    }

    /// Get gesture history
    pub fn get_gesture_history(&self) -> &[GestureData] {
        &self.gesture_history
    }
}

/// NUWE-compatible motion capture node
pub struct MotionCaptureNode {
    pub id: String,
    pub name: String,
    processor: MotionCaptureProcessor,
    mediapipe_enabled: bool,
    leapmotion_enabled: bool,
}

impl MotionCaptureNode {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            processor: MotionCaptureProcessor::new(),
            mediapipe_enabled: true,
            leapmotion_enabled: true,
        }
    }

    /// Process MediaPipe data
    pub fn process_mediapipe(
        &mut self,
        hand_landmarks: Option<&Value>,
        pose_landmarks: Option<&Value>,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        if !self.mediapipe_enabled {
            return Err("MediaPipe integration disabled".into());
        }

        let gesture_data = self.processor.process_mediapipe_data(hand_landmarks, pose_landmarks)?;

        let mut output = HashMap::new();
        output.insert("gesture_type".to_string(), Value::String(gesture_data.gesture_type));
        output.insert("confidence".to_string(), Value::Number(serde_json::Number::from_f64(gesture_data.confidence as f64).unwrap()));
        output.insert("parameters".to_string(), serde_json::to_value(&gesture_data.parameters).unwrap_or(Value::Null));
        output.insert("timestamp".to_string(), Value::Number(serde_json::Number::from_f64(gesture_data.timestamp).unwrap()));

        Ok(output)
    }

    /// Process LeapMotion data
    pub fn process_leapmotion(
        &mut self,
        hand_positions: Option<&Value>,
        finger_positions: Option<&Value>,
        gestures: Option<&Value>,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        if !self.leapmotion_enabled {
            return Err("LeapMotion integration disabled".into());
        }

        let gesture_data = self.processor.process_leapmotion_data(hand_positions, finger_positions, gestures)?;

        let mut output = HashMap::new();
        output.insert("gesture_type".to_string(), Value::String(gesture_data.gesture_type));
        output.insert("confidence".to_string(), Value::Number(serde_json::Number::from_f64(gesture_data.confidence as f64).unwrap()));
        output.insert("parameters".to_string(), serde_json::to_value(&gesture_data.parameters).unwrap_or(Value::Null));
        output.insert("timestamp".to_string(), Value::Number(serde_json::Number::from_f64(gesture_data.timestamp).unwrap()));

        Ok(output)
    }

    /// Configure motion capture settings
    pub fn configure(&mut self, mediapipe_enabled: bool, leapmotion_enabled: bool) {
        self.mediapipe_enabled = mediapipe_enabled;
        self.leapmotion_enabled = leapmotion_enabled;
    }
}