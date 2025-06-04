use crate::event_log::ActionEvent;
use std::sync::{Arc, Mutex};
use std::thread;
use rdev::{listen, Event, EventType};

pub fn start_tracking(logs: Arc<Mutex<Vec<ActionEvent>>>) {
    thread::spawn(move || {
        if let Err(error) = listen(move |event: Event| {
            let details = format!("{:?}", event);
            let action = match event.event_type {
                EventType::KeyPress(_) => "KeyPress",
                EventType::KeyRelease(_) => "KeyRelease",
                EventType::ButtonPress(_) => "MouseDown",
                EventType::ButtonRelease(_) => "MouseUp",
                EventType::MouseMove { .. } => "MouseMove",
                EventType::Wheel { .. } => "MouseWheel",
                _ => "Other",
            };
            let log = ActionEvent::new(action, &details);
            if let Ok(mut logs) = logs.lock() {
                logs.push(log);
            }
        }) {
            eprintln!("Error: {:?}", error);
        }
    });
}