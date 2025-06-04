use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem, MenuEvent}};
use std::sync::mpsc::Sender;
use std::thread;
use tray_icon::icon::Icon;

pub enum TrayEvent {
    ShowWindow,
    Exit,
}

pub fn init_tray(event_sender: Sender<TrayEvent>) {
    thread::spawn(move || {
        let mut menu = Menu::new();
        let show_item = MenuItem::new("Show", true, None);
        let exit_item = MenuItem::new("Exit", true, None);
        menu.append(&show_item);
        menu.append(&exit_item);

        let icon = Icon::from_path("icon.png", None).expect("Failed to load icon");
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Task Tracker")
            .with_icon(icon)
            .build()
            .unwrap();

        let show_id = show_item.id();
        let exit_id = exit_item.id();

        loop {
            if let Ok(event) = MenuEvent::receiver().recv() {
                if event.id == show_id {
                    event_sender.send(TrayEvent::ShowWindow).unwrap();
                } else if event.id == exit_id {
                    event_sender.send(TrayEvent::Exit).unwrap();
                }
            }
        }
    });
}
