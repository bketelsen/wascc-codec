// Copyright 2015-2020 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Common types used for managing native capability providers

use std::error::Error;

use std::any::Any;

/// All capability providers must respond to this operation, which will be requested by
/// the host (the `system` actor)
pub const OP_GET_CAPABILITY_DESCRIPTOR: &str = "GetCapabilityDescriptor";

/// The dispatcher is used by a native capability provider to send commands to an actor module, expecting
/// a result containing a byte array in return
pub trait Dispatcher: Any + Send + Sync {
    fn dispatch(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}

/// Metadata describing the capability provider and the operations it supports
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CapabilityDescriptor {
    /// The capability ID of the provider, e.g. `wascc:messaging` or `thirdparty:someprovider`
    pub id: String,
    /// The human-friendly name of the provider, displayed in short messages and log entries
    pub name: String,
    /// A semver string representing the version of the provider module
    pub version: String,
    /// A monotonicaly increasing revision number
    pub revision: u32,
    /// A longer, documentation-friendly, description of this provider
    pub long_description: String,
    /// A list of all of the operations supported by this provider
    pub supported_operations: Vec<OperationDescriptor>,
}

impl CapabilityDescriptor {
    pub fn builder() -> CapabilityDescriptorBuilder {
        CapabilityDescriptorBuilder::new()
    }
}

/// A fluent syntax builder for creating a capability descriptor
#[derive(Default)]
pub struct CapabilityDescriptorBuilder {
    descriptor: CapabilityDescriptor,
}

impl CapabilityDescriptorBuilder {
    /// Creates a new capability descriptor builder
    fn new() -> CapabilityDescriptorBuilder {
        CapabilityDescriptorBuilder::default()
    }

    /// Sets the capability ID (e.g. `wascc:messaging`) of the provider
    pub fn id(self, id: &str) -> Self {
        CapabilityDescriptorBuilder {
            descriptor: CapabilityDescriptor {
                id: id.to_string(),
                ..self.descriptor
            },
        }
    }

    /// Sets the name of the capability provider.
    pub fn name(self, name: &str) -> Self {
        CapabilityDescriptorBuilder {
            descriptor: CapabilityDescriptor {
                name: name.to_string(),
                ..self.descriptor
            },
        }
    }

    /// Sets a longer, documentation-friendly description of the provider
    pub fn long_description(self, desc: &str) -> Self {
        CapabilityDescriptorBuilder {
            descriptor: CapabilityDescriptor {
                long_description: desc.to_string(),
                ..self.descriptor
            },
        }
    }

    /// Sets the version string (semver by convention) of the provider
    pub fn version(self, ver: &str) -> Self {
        CapabilityDescriptorBuilder {
            descriptor: CapabilityDescriptor {
                version: ver.to_string(),
                ..self.descriptor
            },
        }
    }

    /// Sets the monotonically increasing, numeric revision number of a provider. Used when comparing provider versions
    pub fn revision(self, rev: u32) -> Self {
        CapabilityDescriptorBuilder {
            descriptor: CapabilityDescriptor {
                revision: rev,
                ..self.descriptor
            },
        }
    }

    /// Adds an operation descriptor to the provider descriptor.
    pub fn with_operation(self, name: &str, direction: OperationDirection, doctext: &str) -> Self {
        let mut newops = self.descriptor.supported_operations;
        newops.push(OperationDescriptor::new(name, direction, doctext));
        CapabilityDescriptorBuilder {
            descriptor: CapabilityDescriptor {
                supported_operations: newops,
                ..self.descriptor
            },
        }
    }

    /// Produces a new capability descriptor from the builder's configuration
    pub fn build(self) -> CapabilityDescriptor {
        self.descriptor
    }
}

/// A description of a single operation supported by a capability provider
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OperationDescriptor {
    /// The name of the operation. This must be unique per capability ID
    pub name: String,
    /// Indicates the direction of the operation (can be bi-directional)
    pub direction: OperationDirection,
    /// Documentation-suitable text for this operation
    pub doctext: String,
}

impl OperationDescriptor {
    /// Creates a new operation descriptor
    pub fn new(name: &str, direction: OperationDirection, doctext: &str) -> OperationDescriptor {
        OperationDescriptor {
            name: name.to_string(),
            direction,
            doctext: doctext.to_string(),
        }
    }
}

/// Represents the direction of an operation invocation
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OperationDirection {
    ToActor,
    ToProvider,
    Both,
}

/// The NullDispatcher is as its name implies--a dispatcher that does nothing. This is convenient for
/// initializing a capability provider with a null dispatcher, and then swapping it for a real dispatcher
/// when the host runtime provides one configured with the appropriate channels
#[derive(Default)]
pub struct NullDispatcher {}

impl NullDispatcher {
    pub fn new() -> NullDispatcher {
        NullDispatcher {}
    }
}

impl Dispatcher for NullDispatcher {
    fn dispatch(&self, _actor: &str, _op: &str, _msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        unimplemented!()
    }
}

/// Every native capability provider must implement this trait. Both portable and native capability providers
/// must respond to the following operations: `OP_BIND_ACTOR`, `OP_REMOVE_ACTOR`, `OP_GET_CAPABILITY_DESCRIPTOR`
pub trait CapabilityProvider: Any + Send + Sync {
    /// This function will be called on the provider when the host runtime is ready and has configured a dispatcher. This function is only ever
    /// called _once_ for a capability provider, regardless of the number of actors being managed in the host
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>>;
    /// Invoked when an actor has requested that a provider perform a given operation
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}

/// Wraps a constructor inside an FFI function to allow the `CapabilityProvider` trait implementation
/// to be instantiated and used by the host runtime
#[macro_export]
macro_rules! capability_provider {
    ($provider_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn __capability_provider_create(
        ) -> *mut $crate::capabilities::CapabilityProvider {
            let constructor: fn() -> $provider_type = $constructor;
            let object = constructor();
            let boxed: Box<$crate::capabilities::CapabilityProvider> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

#[cfg(test)]
mod test {
    use super::{CapabilityDescriptor, OperationDescriptor, OperationDirection};
    #[test]
    fn descriptor_certify_desired_json_format() {
        let d = CapabilityDescriptor {
            name: "test".to_string(),
            id: "wascc:testing".to_string(),
            version: "0.0.1".to_string(),
            revision: 1,
            long_description: "this is a test".to_string(),
            supported_operations: vec![OperationDescriptor {
                direction: OperationDirection::ToActor,
                doctext: "this is a test".to_string(),
                name: "OperationDumboDrop".to_string(),
            }],
        };
        let s = serde_json::to_string(&d).unwrap();
        assert_eq!(s, "{\"id\":\"wascc:testing\",\"name\":\"test\",\"version\":\"0.0.1\",\"revision\":1,\"long_description\":\"this is a test\",\"supported_operations\":[{\"name\":\"OperationDumboDrop\",\"direction\":\"to_actor\",\"doctext\":\"this is a test\"}]}".to_string());
    }
}
