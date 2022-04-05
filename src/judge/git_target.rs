use std::fmt::{Debug, Display};

#[derive(Debug,Hash,Eq,PartialEq,Clone)]
pub struct GitTarget {
    pub url: String,
    pub commit: String,
}
impl Display for GitTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitTarget({}::{})", self.get_name(), self.commit)
    }
}
impl GitTarget {
    pub fn repo(url: String) -> Self {
        Self {
            url,
            commit: String::from("HEAD"),
        }
    }
    pub fn on_commit(mut self, commit: String) -> Self {
        self.commit = commit;
        self
    }

    pub fn get_name(&self) -> &str {
        let name = self.url.split('/').last().unwrap();
        name.strip_suffix(".git").unwrap_or(name)
    }
}

