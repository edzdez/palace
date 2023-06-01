#[derive(Debug, Clone)]
pub enum Message {
    Join(String),
    Leave(String),
    EndGame,
}
