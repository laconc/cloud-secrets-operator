use k8s_openapi::api::core::v1::Container;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[kube(
    group = "64f.dev",
    version = "v1alpha1",
    kind = "CloudSecret",
    namespaced,
    status = "CloudSecretStatus"
)]
pub struct CloudSecretSpec {
    pub secret_name: Option<String>,
    pub source: SourceSpec,
    pub strict: Option<bool>,
    pub keys: Option<Vec<KeySpec>>,
    pub refresh_interval: Option<String>, // e.g., "1h", "30m"
    pub config: Option<Vec<KeyConfig>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CloudSecretStatus {
    pub conditions: Vec<CloudSecretStatusCondition>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CloudSecretStatusCondition {
    pub synced: bool,
    pub version_id: Option<String>,
    pub last_sync_time: Option<String>,
    pub last_update_time: String,
    pub message: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SourceSpec {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct KeySpec {
    pub name: String,
    pub rotate_interval: Option<String>, // e.g., "90d"
    pub config: Option<Vec<KeyConfig>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct KeyConfig {
    pub create: Option<ValidationSpec>,
    pub rotate: Option<ValidationSpec>,
    pub validate: Option<ValidationSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ValidationSpec {
    pub regex: Option<String>,
    pub container: Option<Container>,
}

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[kube(
    group = "64f.dev",
    version = "v1alpha1",
    kind = "CloudSecretProvider",
    status = "CloudSecretProviderStatus"
)]
pub struct CloudSecretProviderSpec {
    pub provider: AwsProvider,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CloudSecretProviderStatus {
    pub conditions: Vec<CloudSecretProviderStatusCondition>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CloudSecretProviderStatusCondition {
    pub ready: bool,
    pub last_update_time: String,
    pub message: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AwsProvider {
    pub region: String,
    pub auth: AwsAuth,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AwsAuth {
    pub secret_name: String,
}
