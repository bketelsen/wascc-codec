//! # Core data types
//!
//! This module contains data types used for wascc actor module and host runtime communications
//! that is not specific to any given capability provider

use std::collections::HashMap;

pub const OP_PERFORM_LIVE_UPDATE: &str = "PerformLiveUpdate";
pub const OP_IDENTIFY_CAPABILITY: &str = "IdentifyCapability";
pub const OP_HEALTH_REQUEST: &str = "HealthRequest";
pub const OP_INITIALIZE: &str = "Initialize";
pub const OP_BIND_ACTOR: &str = "BindActor";
pub const OP_REMOVE_ACTOR: &str = "RemoveActor";

// Keys used for providing actor claim data to a capability provider during binding

pub const CONFIG_WASCC_CLAIMS_ISSUER: &str = "__wascc_issuer";
pub const CONFIG_WASCC_CLAIMS_CAPABILITIES: &str = "__wascc_capabilities";
pub const CONFIG_WASCC_CLAIMS_NAME: &str = "__wascc_name";
pub const CONFIG_WASCC_CLAIMS_EXPIRES: &str = "__wascc_expires";
pub const CONFIG_WASCC_CLAIMS_TAGS: &str = "__wascc_tags";

/// LiveUpdate is used when a module is being replaced. The bytes contained in this message will, if valid,
/// replace the existing actor. This message is sent to an actor from the "system" origin
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveUpdate {
    /// Raw bytes of the new actor
    pub new_module: Vec<u8>,
}

/// A health request is passed to an actor to allow it to return an empty result. If the guest module
/// returns the empty result, it is considered healthy. More fields may be added to this message in the future
/// to support more fine-grained health detection
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct HealthRequest {
    /// A placeholder not currently used for health checks
    pub placeholder: bool,
}

/// Capability providers must be able to accept configuration values on a per-actor basis. The module
/// field will be the public key of the actor (the `sub` field of its embedded JWT), though providers
/// should treat this string as opaque data to be used as a key
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CapabilityConfiguration {
    /// The key to be used to distinguish actor configuration, this is the subject's public key
    pub module: String,
    /// Raw configuration values
    #[serde(default)]
    pub values: HashMap<String, String>,
}
