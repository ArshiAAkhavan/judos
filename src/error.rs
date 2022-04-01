pub type Result<T> = std::result::Result<T, PipelineError>;

#[derive(Debug)]
pub enum PipelineError {
    TriggerError,
    MalformedOutput,
}
