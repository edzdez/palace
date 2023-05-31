use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("There is no card at position `{0}`")]
    NoCard(usize),
    #[error("Index `{0}` is invalid")]
    InvalidIndex(usize),
}
