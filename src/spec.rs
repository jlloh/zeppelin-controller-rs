use chrono::{DateTime, Utc};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(kind = "Zeppelin", group = "kube.rs", version = "v1", namespaced)]
#[kube(status = "ZeppelinStatus")]
pub struct ZeppelinSpec {
    // App name used for deployment name, etc
    pub(crate) name: String,
    server_specs: ServerSpec,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ServerSpec {
    replicas: i64,
    cpu: String,
    memory: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ZeppelinStatus {
    last_updated: Option<DateTime<Utc>>,
}
