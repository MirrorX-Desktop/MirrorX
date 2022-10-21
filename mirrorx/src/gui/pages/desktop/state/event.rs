use strum_macros::AsRefStr;

#[derive(Debug, AsRefStr)]
pub enum Event {
    ConnectEndPoint {},
}
