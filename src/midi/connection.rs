use midir::{MidiInputConnection, MidiOutputConnection};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MidiEvent {
    pub timestamp_us: u64,
    pub bytes: Vec<u8>,
}

pub const MONITOR_BUFFER_CAP: usize = 1000;

pub struct MidiMonitor {
    pub input_conn: Option<MidiInputConnection<()>>,
    pub input_port_name: Option<String>,
    pub available_input_ports: Vec<String>,
    pub events: Arc<Mutex<Vec<MidiEvent>>>,
    pub paused: Arc<AtomicBool>,
    pub katana_only: bool,
    pub editor_mode_on: bool,
}

impl Default for MidiMonitor {
    fn default() -> Self {
        Self {
            input_conn: None,
            input_port_name: None,
            available_input_ports: vec![],
            events: Arc::new(Mutex::new(Vec::with_capacity(MONITOR_BUFFER_CAP))),
            paused: Arc::new(AtomicBool::new(false)),
            katana_only: true,
            editor_mode_on: false,
        }
    }
}

impl MidiMonitor {
    pub fn clear_events(&self) {
        if let Ok(mut e) = self.events.lock() { e.clear(); }
    }
    
    pub fn is_listening(&self) -> bool {
        self.input_conn.is_some() && !self.paused.load(Ordering::Relaxed)
    }
}