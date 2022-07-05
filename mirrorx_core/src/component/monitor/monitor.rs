#[derive(Debug)]
pub struct Monitor {
    pub id: String,
    pub name: String,
    pub refresh_rate: String,
    pub width: u16,
    pub height: u16,
    pub main: bool,
    pub screen_shot: Vec<u8>,
}
