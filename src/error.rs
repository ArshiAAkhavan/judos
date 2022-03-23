pub type Result<T> = std::result::Result<T, PipelineError>;

pub enum PipelineError {
    TriggerError,
    MalformedOutput,
}
