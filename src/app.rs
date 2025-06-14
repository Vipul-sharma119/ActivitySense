use crate::{event_log::ActionEvent, history::TaskSummary, tracker::start_tracking, tray::{init_tray, TrayEvent}};
use chrono::Local;
use csv::Writer;
use eframe::{egui, CreationContext,NativeOptions};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver};
use opener;

pub struct TaskTrackerApp {
    task_name: String,
    is_tracking: bool,
    logs: Arc<Mutex<Vec<ActionEvent>>>,
    history: Vec<TaskSummary>,
    tray_receiver: Receiver<TrayEvent>,
}

impl TaskTrackerApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        let (sender, receiver) = mpsc::channel();
        init_tray(sender);
        Self {
            task_name: String::new(),
            is_tracking: false,
            logs: Arc::new(Mutex::new(Vec::new())),
            history: Vec::new(),
            tray_receiver: receiver,
        }
    }
//functions helps in exporting csv
    fn export_to_csv(&mut self) {
        let now = Local::now();
        let filename = format!("{}_{}.csv", self.task_name, now.format("%Y-%m-%d_%H-%M-%S"));
        let file = File::create(&filename).expect("Cannot create file");
        let mut writer = Writer::from_writer(file);

        writer
            .write_record(&["timestamp", "event_type", "details"])
            .unwrap();

        let logs = self.logs.lock().unwrap();
        for log in logs.iter() {
            writer
                .write_record(&[&log.timestamp, &log.event_type, &log.details])
                .unwrap();
        }

        self.history.push(TaskSummary {
            task_name: self.task_name.clone(),
            filename: filename.clone(),
            total_events: logs.len(),
            timestamp: now,
        });

        println!("Task saved to {}", filename);
    }
}

impl eframe::App for TaskTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.tray_receiver.try_recv() {
            match event {
                TrayEvent::ShowWindow => {
                    
                }
                TrayEvent::Exit => {
                    std::process::exit(0);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Tracker");

            ui.horizontal(|ui| {
                ui.label("Task Name:");
                ui.text_edit_singleline(&mut self.task_name);
            });

            if ui.button("Start Task").clicked() && !self.is_tracking {
                self.is_tracking = true;
                let logs = self.logs.clone();
                logs.lock().unwrap().clear();
                start_tracking(logs);
            }

            if ui.button("Stop Task").clicked() && self.is_tracking {
                self.is_tracking = false;
                self.export_to_csv();
            }

            if self.is_tracking {
                ui.label("Tracking... (Press 'Stop Task' to finish)");
            }

            ui.separator();
            ui.heading("Task History");

            egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                for task in self.history.iter().rev() {
                    ui.horizontal(|ui| {
                        ui.label(task.display_string());
                        if ui.button("Show CSV").clicked() {
                            if let Err(e) = opener::open(&task.filename) {
                                eprintln!("Failed to open file: {}", e);
                            }
                        }
                    });
                }
            });
        });
    }
}
