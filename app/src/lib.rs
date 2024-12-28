use k8s_openapi::api::core::v1::Container;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[kube(
    group = "64f.dev",
    version = "v1alpha1",
    kind = "CloudSecret",
    namespaced,
    status = "CloudSecretStatus",
    shortname = "cs",
    printcolumn = r#"{
        "name": "Last Sync Time",
        "type": "string",
        "jsonPath": ".status.lastSyncTime"
    }"#,
    printcolumn = r#"{
        "name": "Status",
        "type": "string",
        "jsonPath": ".status.conditions[?(@.type == 'Synced')].reason"
    }"#
)]
pub struct CloudSecretSpec {
    /// Optional name for the Kubernetes Secret; if not provided, the CloudSecret name is used.
    pub secret_name: Option<String>,

    /// Description of the secret.
    pub description: Option<String>,

    /// Configuration of the source for the secret data.
    pub source: SourceSpec,

    /// Whether the source secret must only contain the keys specified.
    #[serde(default)]
    pub strict: Option<bool>,

    /// Optional list of keys and actions that can be applied to them.
    /// If an action is set for a key, it will override the action specified at the secret level.
    pub keys: Option<Vec<KeySpec>>,

    /// Refresh interval for syncing the secret data, e.g., '3m' or '1h'.
    #[serde(default = "default_refresh_interval")]
    #[schemars(regex(pattern = r"^\d+[mhd]$"))]
    pub refresh_interval: Option<String>,

    /// Actions to perform on the secret keys. These actions apply to all the keys.
    pub actions: Option<ActionsSpec>,
}

fn default_refresh_interval() -> Option<String> {
    Some("3m".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SourceSpec {
    /// Identifier used in the source provider.
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeySpec {
    /// Name of the key in the source provider.
    pub name: String,

    /// Optional new name to use in the Kubernetes Secret. If not provided, the source name is used.
    pub target_name: Option<String>,

    /// Description of the key.
    pub description: Option<String>,

    /// Rotation interval for this key, e.g., '90d'. If present, the operator will rotate
    /// this key at the specified interval.
    #[schemars(regex(pattern = r"^\d+[mhd]$"))]
    pub rotate_interval: Option<String>,

    /// Actions to perform on the key.
    pub actions: Option<ActionsSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActionsSpec {
    /// If the key isn't present, create it according to the specified pattern or logic.
    pub create: Option<ActionSpec>,

    /// Rotate the key according to the specified pattern or logic.
    pub rotate: Option<ActionSpec>,

    /// Validate the key according to the specified pattern or logic.
    pub validate: Option<ActionSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActionSpec {
    /// Regex pattern for the key's value.
    pub pattern: Option<String>,

    /// Container specification for external validation logic on the key.
    pub container: Option<Container>,

    /// Minimum length for the key's value.
    #[schemars(range(min = 0))]
    pub minimum: Option<u64>,

    /// Maximum length for the key's value.
    #[schemars(range(min = 1))]
    pub maximum: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloudSecretStatus {
    /// Name of the managed Kubernetes Secret.
    pub target_secret_name: String,

    /// List of conditions describing the current state of the CloudSecret.
    pub conditions: Vec<CloudSecretStatusCondition>,

    /// Last time the secret was successfully synced.
    pub last_sync_time: Option<String>,

    /// Version ID of the source secret.
    pub version_id: Option<String>,
}

// Reference: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#typical-status-properties
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloudSecretStatusCondition {
    /// Type of the condition.
    pub type_: CloudSecretStatusType,

    /// Status of the condition.
    pub status: ConditionStatus,

    /// Generation observed when the condition was set.
    pub observed_generation: i64,

    /// Last time this condition transitioned.
    pub last_transition_time: String,

    /// Human-readable message indicating details about the transition.
    pub message: String,

    /// Reason for the condition's last transition.
    pub reason: Option<CloudSecretStatusReason>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum ConditionStatus {
    True,
    False,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum CloudSecretStatusType {
    Synced,
    Reconciling,
    Applying,
    Rotating,
    Validating,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum CloudSecretStatusReason {
    ValidationSucceeded,
    ValidationFailed,
    SecretApplied,
    SecretRotated,
    SourceUnavailable,
    ProviderError,
}

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[kube(
    group = "64f.dev",
    version = "v1alpha1",
    kind = "CloudSecretProvider",
    status = "CloudSecretProviderStatus",
    shortname = "csp",
    printcolumn = r#"{
        "name": "Provider Type",
        "type": "string",
        "jsonPath": ".spec.provider"
    }"#,
    printcolumn = r#"{
        "name": "Ready",
        "type": "string",
        "jsonPath": ".status.conditions[?(@.type == 'Ready')].status"
    }"#
)]
pub struct CloudSecretProviderSpec {
    /// Description of the provider.
    pub description: Option<String>,

    /// Configuration for the secrets provider.
    pub provider: ProviderSpec,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloudSecretProviderStatus {
    pub conditions: Vec<CloudSecretProviderStatusCondition>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloudSecretProviderStatusCondition {
    /// Type of the condition.
    pub type_: CloudSecretProviderStatusType,

    /// Status of the condition.
    pub status: ConditionStatus,

    /// Generation observed when the condition was set.
    pub observed_generation: i64,

    /// Last time this condition transitioned.
    pub last_transition_time: String,

    /// Human-readable message indicating details about the transition.
    pub message: String,

    /// Reason for the condition's last transition.
    pub reason: Option<CloudSecretProviderStatusReason>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum CloudSecretProviderStatusType {
    Ready,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum CloudSecretProviderStatusReason {
    AuthenticationSucceeded,
    AuthenticationFailed,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ProviderSpec {
    /// Configuration for AWS Secrets Manager.
    AwsSecretsManager {
        /// AWS region.
        region: String,
        /// Optional authentication configuration for AWS.
        auth: Option<AwsAuthConfig>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AwsAuthConfig {
    /// Optional name of the Kubernetes Secret containing the AWS credentials.
    pub secret_name: Option<String>,
    /// Optional IRSA configuration.
    pub irsa: Option<IrsaConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IrsaConfig {
    /// Name of the Kubernetes ServiceAccount to use for IRSA.
    pub secret_name: Option<String>,
    /// ARN of the IAM role to assume.
    pub role_arn: Option<String>,
}
