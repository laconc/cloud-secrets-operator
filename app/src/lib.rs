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

    /// Configuration of the source for the secret data.
    pub source: SourceSpec,

    /// Whether the source secret must only contain the keys specified.
    #[serde(default)]
    pub strict: Option<bool>,

    /// Optional list of keys with actions that should be applied to each key.
    /// If an action is set for a key, it will override the action specified at the global level.
    pub keys: Option<Vec<KeySpec>>,

    /// Refresh interval for syncing the secret data, e.g., '3m' or '1h'.
    #[serde(default = "default_refresh_interval")]
    #[schemars(regex(pattern = r"^\d+[mhd]$"))]
    pub refresh_interval: Option<String>,

    /// Actions to perform on the secret keys. These are global actions that apply to all keys.
    pub actions: Option<ActionsSpec>,
}

fn default_refresh_interval() -> Option<String> {
    Some("3m".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SourceSpec {
    /// The identifier used in the source provider.
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeySpec {
    /// The name of the key in the source provider.
    pub name: String,

    /// Optional new name for the key to use in the Kubernetes Secret. If not provided, the source name is used.
    pub new_name: Option<String>,

    /// Rotation interval for this key, e.g., '90d'. If specified, the operator will rotate this key according to the defined interval.
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
    /// A regex pattern for the key's value.
    pub pattern: Option<String>,

    /// Container specification for external validation logic on the key.
    pub container: Option<Container>,

    /// The minimum length for the key's value.
    #[schemars(range(min = 0))]
    pub minimum: Option<u64>,

    /// The maximum length for the key's value.
    #[schemars(range(min = 1))]
    pub maximum: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloudSecretStatus {
    /// List of conditions describing the current state of the CloudSecret.
    pub conditions: Vec<CloudSecretStatusCondition>,

    /// The last time the secret was successfully synced.
    pub last_sync_time: Option<String>,

    /// The version ID of the source secret.
    pub version_id: Option<String>,
}

// Reference: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#typical-status-properties
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloudSecretStatusCondition {
    /// The type of the condition.
    pub type_: CloudSecretStatusType,

    /// The status of the condition. Valid values are 'True', 'False', or 'Unknown'.
    pub status: ConditionStatus,

    /// The generation observed when the condition was set.
    pub observed_generation: i64,

    /// The last time this condition transitioned.
    pub last_transition_time: String,

    /// A human-readable message indicating details about the transition.
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
    pub type_: CloudSecretProviderStatusType,
    pub status: ConditionStatus,
    pub observed_generation: i64,
    pub last_transition_time: String,
    pub message: String,
    pub reason: Option<CloudSecretProviderStatusReason>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum CloudSecretProviderStatusType {
    Ready,
    Reconciling,
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
        /// The AWS region.
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
    /// The name of the Kubernetes ServiceAccount to use for IRSA.
    pub secret_name: Option<String>,
    /// The ARN of the IAM role to assume.
    pub role_arn: Option<String>,
}
