use std::{fmt::Debug, path::Path};
use super::GitTarget;
use crate::error::Result;

#[typetag::serde]
pub trait Judge: Send + Sync + Debug {
    fn judge(&self, target: &GitTarget, from_path: &Path) -> Result<f64>;
}
