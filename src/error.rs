/// comment 1
pub type Result<T> = std::result::Result<T, JudosError>;


#[derive(Debug)]
pub enum JudosError {
    ///comment 4
    TriggerError,
    ///comment 5
    MalformedOutput,
}

