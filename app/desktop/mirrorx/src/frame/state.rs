use super::{
    color::{ThemeColor, ThemeColorStyle},
    view::ViewId,
};
use crossbeam::atomic::AtomicCell;
use std::{
    rc::Rc,
    sync::{atomic::AtomicPtr, Arc},
    time::Instant,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub enum UIEvent {
    UpdateThemeColorStyle(ThemeColorStyle),
}

impl UIEvent {
    pub fn name(&self) -> &str {
        match self {
            UIEvent::UpdateThemeColorStyle(_) => "UpdateThemeColorStyle",
        }
    }
}

#[derive(Clone)]
pub enum MyDeviceType {
    Computer,
    Phone,
}

#[derive(Clone)]
pub struct MyDevice {
    pub name: String,
    pub device_type: MyDeviceType,
    pub is_this_computer: bool,
}

#[derive(Clone)]
pub enum Peer {
    Temporary {
        id: u32,
        domain: String,
    },
    Registered {
        id: u32,
        nickname: String,
        domain: String,
    },
}

pub struct SharedState {
    tx: UnboundedSender<UIEvent>,
    theme_color: &'static ThemeColor,
    current_view_id: ViewId,
    peer: Peer,
    use_totp: bool,
    totp: String,
    use_otp: bool,
    otp: String,
    use_permanent_password: bool,
    permanent_password: String,
    my_devices: Vec<MyDevice>,
    notifications: NotificationHub,
}

impl SharedState {
    pub fn new(tx: UnboundedSender<UIEvent>) -> Self {
        // Self {
        //     tx,
        //     theme_color: ThemeColor::select_style(&ThemeColorStyle::Light),

        //     use_totp: true,
        //     totp: String::from("ABCDEF"),
        //     use_otp: true,
        //     otp: String::from("ABCDEF"),
        //     use_permanent_password: false,

        //     permanent_password: String::from("AABBVV"),
        //     my_devices: Vec::new(),

        //     notifications: NotificationHub::new(),
        // }
        todo!()
    }

    pub fn theme_color(&self) -> &ThemeColor {
        self.theme_color
    }

    pub fn set_theme_color(&self, theme_color_style: ThemeColorStyle) {
        self.send_ui_event(UIEvent::UpdateThemeColorStyle(theme_color_style));
    }

    pub fn current_view_id(&self) -> ViewId {
        self.current_view_id
    }

    pub fn send_ui_event(&self, event: UIEvent) {
        if let Err(event) = self.tx.send(event) {
            tracing::error!(name = event.0.name(), "send ui event failed");
        }
    }
}

#[derive(Clone)]
pub struct Notification {
    pub content: String,
    pub ts: Instant,
}

#[derive(Clone)]
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

pub fn start_ui_event_processor(atomic_state: AtomicCell<SharedState>) {
    let mut state = SharedState::default();
    let (tx, rx) = unbounded_channel();
    tokio::spawn(async move {
        loop {
            let Some(event) = rx.recv().await else {
                tracing::info!("ui event processor exit");
                return;
            };

            // update shared ui_event
            atomic_state.swap(state.clone());
        }
    });
}
