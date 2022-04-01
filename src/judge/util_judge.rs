use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{GitTarget, Judge};
use crate::error::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct DummyJudge;

#[typetag::serde(name = "dummy")]
impl Judge for DummyJudge {
    fn judge(&self, _target: &GitTarget, _from_path: &Path) -> Result<f64> {
        Ok(0.0f64)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConstJudge {
    score: f64,
}

#[typetag::serde(name = "const")]
impl Judge for ConstJudge {
    fn judge(&self, _target: &GitTarget, _from_path: &Path) -> Result<f64> {
        Ok(self.score)
    }
}
