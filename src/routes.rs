//! routes.json model (CC 62001 §4). Three route kinds: resource
//! (static file), dataset (REST route), handler (dynamic JS).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RouteKind {
    Resource,
    Dataset,
    Handler,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    #[serde(default)]
    pub resource: Option<String>,
    #[serde(default)]
    pub dataset: Option<String>,
    #[serde(default)]
    pub handler: Option<String>,
    #[serde(default, rename = "method")]
    pub http_method: Option<String>,
}

impl Route {
    /// Discriminate the route kind by which target field is set.
    /// Mirrors the Ruby reactor's Route#kind logic so both runtimes
    /// agree on dispatch.
    pub fn kind(&self) -> RouteKind {
        if self.resource.is_some() {
            RouteKind::Resource
        } else if self.dataset.is_some() {
            RouteKind::Dataset
        } else {
            RouteKind::Handler
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routes {
    #[serde(default)]
    pub index: Option<String>,
    #[serde(default)]
    pub routes: Vec<Route>,
}

impl Routes {
    pub fn from_json(text: &str) -> Result<Self, crate::PackageError> {
        serde_json::from_str(text).map_err(crate::PackageError::from)
    }

    /// Find the route declaring this serving path, or None.
    pub fn resolve(&self, path: &str) -> Option<&Route> {
        self.routes.iter().find(|r| r.path == path)
    }
}
