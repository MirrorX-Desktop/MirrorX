use std::{collections::VecDeque, time::Instant};

use once_cell::sync::{Lazy, OnceCell};
use tokio::sync::mpsc::UnboundedReceiver;

pub enum UIEvent {}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PageType {
    Home,
    Lan,
    History,
    Settings,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ConnectType {
    Desktop,
    Files,
}

pub enum MyDeviceType {
    Computer,
    Phone,
}

pub struct MyDevice {
    pub name: String,
    pub device_type: MyDeviceType,
    pub is_this_computer: bool,
}

pub struct UIState {
    pub current_page_type: PageType,
    pub connect_type: ConnectType,
    pub peer_id: String,
    pub peer_domain: String,
    pub peer_connect_content: String,
    pub use_totp_password: bool,
    pub totp_password: String,
    pub use_otp_password: bool,
    pub otp_password: String,
    pub use_permanent_password: bool,
    pub permanent_password: String,
    pub is_login: bool,
    pub my_devices: Vec<MyDevice>,
    pub login_email: String,
    pub login_password: String,
    pub notifications: NotificationHub,
}

impl UIState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            current_page_type: PageType::Home,
            connect_type: ConnectType::Desktop,
            peer_id: String::default(),
            peer_domain: String::from("mirrorx.cloud"),
            peer_connect_content: String::default(),
            use_totp_password: true,
            totp_password: String::from("ABCDEF"),
            use_otp_password: true,
            otp_password: String::from("ABCDEF"),
            use_permanent_password: false,
            is_login: false,
            permanent_password: String::from("AABBVV"),
            my_devices: Vec::new(),
            login_email: String::default(),
            login_password: String::default(),
            notifications: NotificationHub::new(),
        })
    }
}

pub fn update_ui_state(ui_state: &mut UIState, ui_event_rx: &mut UnboundedReceiver<UIEvent>) {}

pub struct Notification {
    pub content: String,
    pub ts: Instant,
}

pub struct NotificationHub {
    notifications: Vec<Notification>,
}

impl NotificationHub {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    pub fn push_notification(&mut self, content: String) {
        self.notifications.push(Notification {
            content,
            ts: Instant::now(),
        })
    }

    pub fn poll_notifications(&mut self) -> &[Notification] {
        while let Some(notification) = self.notifications.first() {
            if Instant::now().duration_since(notification.ts).as_secs() >= 5 {
                self.notifications.remove(0);
            } else {
                break;
            }
        }

        &self.notifications
    }
}
