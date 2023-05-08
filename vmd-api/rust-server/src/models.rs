#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorModel {
    #[serde(rename = "code")]
    pub code: serde_json::Value,

    #[serde(rename = "message")]
    pub message: serde_json::Value,

}

impl ErrorModel {
    #[allow(clippy::new_without_default)]
    pub fn new(code: serde_json::Value, message: serde_json::Value, ) -> ErrorModel {
        ErrorModel {
            code,
            message,
        }
    }
}

/// Converts the ErrorModel value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ErrorModel {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping code in query parameter serialization

            // Skipping message in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorModel value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorModel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<serde_json::Value>,
            pub message: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ErrorModel".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ErrorModel".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorModel {
            code: intermediate_rep.code.into_iter().next().ok_or_else(|| "code missing in ErrorModel".to_string())?,
            message: intermediate_rep.message.into_iter().next().ok_or_else(|| "message missing in ErrorModel".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorModel> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorModel>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ErrorModel>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ErrorModel - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ErrorModel> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ErrorModel as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ErrorModel - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VirtualMachineBasicDetails {
    #[serde(rename = "id")]
    pub id: serde_json::Value,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "state")]
    pub state: serde_json::Value,

    #[serde(rename = "name")]
    pub name: serde_json::Value,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<serde_json::Value>,

}

impl VirtualMachineBasicDetails {
    #[allow(clippy::new_without_default)]
    pub fn new(id: serde_json::Value, state: serde_json::Value, name: serde_json::Value, ) -> VirtualMachineBasicDetails {
        VirtualMachineBasicDetails {
            id,
            state,
            name,
            description: None,
        }
    }
}

/// Converts the VirtualMachineBasicDetails value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for VirtualMachineBasicDetails {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization

            // Skipping state in query parameter serialization

            // Skipping name in query parameter serialization

            // Skipping description in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VirtualMachineBasicDetails value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VirtualMachineBasicDetails {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<serde_json::Value>,
            pub state: Vec<serde_json::Value>,
            pub name: Vec<serde_json::Value>,
            pub description: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VirtualMachineBasicDetails".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "state" => intermediate_rep.state.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VirtualMachineBasicDetails".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VirtualMachineBasicDetails {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in VirtualMachineBasicDetails".to_string())?,
            state: intermediate_rep.state.into_iter().next().ok_or_else(|| "state missing in VirtualMachineBasicDetails".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in VirtualMachineBasicDetails".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VirtualMachineBasicDetails> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<VirtualMachineBasicDetails>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VirtualMachineBasicDetails>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for VirtualMachineBasicDetails - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<VirtualMachineBasicDetails> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VirtualMachineBasicDetails as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into VirtualMachineBasicDetails - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VirtualMachineCpu {
    #[serde(rename = "model")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub model: Option<serde_json::Value>,

    #[serde(rename = "logicalCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logical_count: Option<serde_json::Value>,

    #[serde(rename = "sockets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sockets: Option<serde_json::Value>,

    #[serde(rename = "cores")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cores: Option<serde_json::Value>,

    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<serde_json::Value>,

}

impl VirtualMachineCpu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> VirtualMachineCpu {
        VirtualMachineCpu {
            model: None,
            logical_count: None,
            sockets: None,
            cores: None,
            threads: None,
        }
    }
}

/// Converts the VirtualMachineCpu value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for VirtualMachineCpu {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping model in query parameter serialization

            // Skipping logicalCount in query parameter serialization

            // Skipping sockets in query parameter serialization

            // Skipping cores in query parameter serialization

            // Skipping threads in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VirtualMachineCpu value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VirtualMachineCpu {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub model: Vec<serde_json::Value>,
            pub logical_count: Vec<serde_json::Value>,
            pub sockets: Vec<serde_json::Value>,
            pub cores: Vec<serde_json::Value>,
            pub threads: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VirtualMachineCpu".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "model" => intermediate_rep.model.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "logicalCount" => intermediate_rep.logical_count.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sockets" => intermediate_rep.sockets.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cores" => intermediate_rep.cores.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "threads" => intermediate_rep.threads.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VirtualMachineCpu".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VirtualMachineCpu {
            model: intermediate_rep.model.into_iter().next(),
            logical_count: intermediate_rep.logical_count.into_iter().next(),
            sockets: intermediate_rep.sockets.into_iter().next(),
            cores: intermediate_rep.cores.into_iter().next(),
            threads: intermediate_rep.threads.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VirtualMachineCpu> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<VirtualMachineCpu>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VirtualMachineCpu>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for VirtualMachineCpu - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<VirtualMachineCpu> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VirtualMachineCpu as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into VirtualMachineCpu - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VirtualMachineDevices {
    #[serde(rename = "list")]
    pub list: serde_json::Value,

}

impl VirtualMachineDevices {
    #[allow(clippy::new_without_default)]
    pub fn new(list: serde_json::Value, ) -> VirtualMachineDevices {
        VirtualMachineDevices {
            list,
        }
    }
}

/// Converts the VirtualMachineDevices value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for VirtualMachineDevices {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping list in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VirtualMachineDevices value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VirtualMachineDevices {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub list: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VirtualMachineDevices".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "list" => intermediate_rep.list.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VirtualMachineDevices".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VirtualMachineDevices {
            list: intermediate_rep.list.into_iter().next().ok_or_else(|| "list missing in VirtualMachineDevices".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VirtualMachineDevices> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<VirtualMachineDevices>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VirtualMachineDevices>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for VirtualMachineDevices - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<VirtualMachineDevices> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VirtualMachineDevices as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into VirtualMachineDevices - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VirtualMachineHypervisorDetails {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "hypervisor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hypervisor: Option<serde_json::Value>,

    #[serde(rename = "arch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arch: Option<serde_json::Value>,

    #[serde(rename = "emulator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emulator: Option<serde_json::Value>,

    #[serde(rename = "chipset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chipset: Option<serde_json::Value>,

    #[serde(rename = "firmware")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub firmware: Option<serde_json::Value>,

}

impl VirtualMachineHypervisorDetails {
    #[allow(clippy::new_without_default)]
    pub fn new() -> VirtualMachineHypervisorDetails {
        VirtualMachineHypervisorDetails {
            hypervisor: None,
            arch: None,
            emulator: None,
            chipset: None,
            firmware: None,
        }
    }
}

/// Converts the VirtualMachineHypervisorDetails value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for VirtualMachineHypervisorDetails {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping hypervisor in query parameter serialization

            // Skipping arch in query parameter serialization

            // Skipping emulator in query parameter serialization

            // Skipping chipset in query parameter serialization

            // Skipping firmware in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VirtualMachineHypervisorDetails value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VirtualMachineHypervisorDetails {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub hypervisor: Vec<serde_json::Value>,
            pub arch: Vec<serde_json::Value>,
            pub emulator: Vec<serde_json::Value>,
            pub chipset: Vec<serde_json::Value>,
            pub firmware: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VirtualMachineHypervisorDetails".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "hypervisor" => intermediate_rep.hypervisor.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "arch" => intermediate_rep.arch.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "emulator" => intermediate_rep.emulator.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "chipset" => intermediate_rep.chipset.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "firmware" => intermediate_rep.firmware.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VirtualMachineHypervisorDetails".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VirtualMachineHypervisorDetails {
            hypervisor: intermediate_rep.hypervisor.into_iter().next(),
            arch: intermediate_rep.arch.into_iter().next(),
            emulator: intermediate_rep.emulator.into_iter().next(),
            chipset: intermediate_rep.chipset.into_iter().next(),
            firmware: intermediate_rep.firmware.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VirtualMachineHypervisorDetails> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<VirtualMachineHypervisorDetails>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VirtualMachineHypervisorDetails>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for VirtualMachineHypervisorDetails - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<VirtualMachineHypervisorDetails> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VirtualMachineHypervisorDetails as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into VirtualMachineHypervisorDetails - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VirtualMachineInfo {
    #[serde(rename = "basicDetails")]
    pub basic_details: models::VirtualMachineBasicDetails,

    #[serde(rename = "hypervisorDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hypervisor_details: Option<models::VirtualMachineHypervisorDetails>,

    #[serde(rename = "cpu")]
    pub cpu: models::VirtualMachineCpu,

    #[serde(rename = "memory")]
    pub memory: models::VirtualMachineMemory,

    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<serde_json::Value>,

    #[serde(rename = "runningTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running_time: Option<serde_json::Value>,

    #[serde(rename = "os")]
    pub os: models::VirtualMachineOs,

    #[serde(rename = "performance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub performance: Option<models::VirtualMachinePerformance>,

    #[serde(rename = "devices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub devices: Option<models::VirtualMachineDevices>,

}

impl VirtualMachineInfo {
    #[allow(clippy::new_without_default)]
    pub fn new(basic_details: models::VirtualMachineBasicDetails, cpu: models::VirtualMachineCpu, memory: models::VirtualMachineMemory, os: models::VirtualMachineOs, ) -> VirtualMachineInfo {
        VirtualMachineInfo {
            basic_details,
            hypervisor_details: None,
            cpu,
            memory,
            start_time: None,
            running_time: None,
            os,
            performance: None,
            devices: None,
        }
    }
}

/// Converts the VirtualMachineInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for VirtualMachineInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping basicDetails in query parameter serialization

            // Skipping hypervisorDetails in query parameter serialization

            // Skipping cpu in query parameter serialization

            // Skipping memory in query parameter serialization

            // Skipping startTime in query parameter serialization

            // Skipping runningTime in query parameter serialization

            // Skipping os in query parameter serialization

            // Skipping performance in query parameter serialization

            // Skipping devices in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VirtualMachineInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VirtualMachineInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub basic_details: Vec<models::VirtualMachineBasicDetails>,
            pub hypervisor_details: Vec<models::VirtualMachineHypervisorDetails>,
            pub cpu: Vec<models::VirtualMachineCpu>,
            pub memory: Vec<models::VirtualMachineMemory>,
            pub start_time: Vec<serde_json::Value>,
            pub running_time: Vec<serde_json::Value>,
            pub os: Vec<models::VirtualMachineOs>,
            pub performance: Vec<models::VirtualMachinePerformance>,
            pub devices: Vec<models::VirtualMachineDevices>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VirtualMachineInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "basicDetails" => intermediate_rep.basic_details.push(<models::VirtualMachineBasicDetails as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hypervisorDetails" => intermediate_rep.hypervisor_details.push(<models::VirtualMachineHypervisorDetails as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cpu" => intermediate_rep.cpu.push(<models::VirtualMachineCpu as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "memory" => intermediate_rep.memory.push(<models::VirtualMachineMemory as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "startTime" => intermediate_rep.start_time.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "runningTime" => intermediate_rep.running_time.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "os" => intermediate_rep.os.push(<models::VirtualMachineOs as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "performance" => intermediate_rep.performance.push(<models::VirtualMachinePerformance as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "devices" => intermediate_rep.devices.push(<models::VirtualMachineDevices as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VirtualMachineInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VirtualMachineInfo {
            basic_details: intermediate_rep.basic_details.into_iter().next().ok_or_else(|| "basicDetails missing in VirtualMachineInfo".to_string())?,
            hypervisor_details: intermediate_rep.hypervisor_details.into_iter().next(),
            cpu: intermediate_rep.cpu.into_iter().next().ok_or_else(|| "cpu missing in VirtualMachineInfo".to_string())?,
            memory: intermediate_rep.memory.into_iter().next().ok_or_else(|| "memory missing in VirtualMachineInfo".to_string())?,
            start_time: intermediate_rep.start_time.into_iter().next(),
            running_time: intermediate_rep.running_time.into_iter().next(),
            os: intermediate_rep.os.into_iter().next().ok_or_else(|| "os missing in VirtualMachineInfo".to_string())?,
            performance: intermediate_rep.performance.into_iter().next(),
            devices: intermediate_rep.devices.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VirtualMachineInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<VirtualMachineInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VirtualMachineInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for VirtualMachineInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<VirtualMachineInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VirtualMachineInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into VirtualMachineInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VirtualMachineMemory {
    /// Total host memory in MB
    #[serde(rename = "totalHost")]
    pub total_host: serde_json::Value,

    /// Currently allocated memory by VM in MB
    #[serde(rename = "currentAllocation")]
    pub current_allocation: serde_json::Value,

    /// Maximum possible allocated memory by VM in MB
    #[serde(rename = "maximumAllocation")]
    pub maximum_allocation: serde_json::Value,

}

impl VirtualMachineMemory {
    #[allow(clippy::new_without_default)]
    pub fn new(total_host: serde_json::Value, current_allocation: serde_json::Value, maximum_allocation: serde_json::Value, ) -> VirtualMachineMemory {
        VirtualMachineMemory {
            total_host,
            current_allocation,
            maximum_allocation,
        }
    }
}

/// Converts the VirtualMachineMemory value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for VirtualMachineMemory {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping totalHost in query parameter serialization

            // Skipping currentAllocation in query parameter serialization

            // Skipping maximumAllocation in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VirtualMachineMemory value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VirtualMachineMemory {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub total_host: Vec<serde_json::Value>,
            pub current_allocation: Vec<serde_json::Value>,
            pub maximum_allocation: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VirtualMachineMemory".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "totalHost" => intermediate_rep.total_host.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "currentAllocation" => intermediate_rep.current_allocation.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maximumAllocation" => intermediate_rep.maximum_allocation.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VirtualMachineMemory".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VirtualMachineMemory {
            total_host: intermediate_rep.total_host.into_iter().next().ok_or_else(|| "totalHost missing in VirtualMachineMemory".to_string())?,
            current_allocation: intermediate_rep.current_allocation.into_iter().next().ok_or_else(|| "currentAllocation missing in VirtualMachineMemory".to_string())?,
            maximum_allocation: intermediate_rep.maximum_allocation.into_iter().next().ok_or_else(|| "maximumAllocation missing in VirtualMachineMemory".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VirtualMachineMemory> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<VirtualMachineMemory>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VirtualMachineMemory>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for VirtualMachineMemory - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<VirtualMachineMemory> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VirtualMachineMemory as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into VirtualMachineMemory - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VirtualMachineOs {
    #[serde(rename = "name")]
    pub name: serde_json::Value,

    #[serde(rename = "version")]
    pub version: serde_json::Value,

}

impl VirtualMachineOs {
    #[allow(clippy::new_without_default)]
    pub fn new(name: serde_json::Value, version: serde_json::Value, ) -> VirtualMachineOs {
        VirtualMachineOs {
            name,
            version,
        }
    }
}

/// Converts the VirtualMachineOs value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for VirtualMachineOs {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping name in query parameter serialization

            // Skipping version in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VirtualMachineOs value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VirtualMachineOs {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<serde_json::Value>,
            pub version: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VirtualMachineOs".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VirtualMachineOs".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VirtualMachineOs {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in VirtualMachineOs".to_string())?,
            version: intermediate_rep.version.into_iter().next().ok_or_else(|| "version missing in VirtualMachineOs".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VirtualMachineOs> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<VirtualMachineOs>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VirtualMachineOs>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for VirtualMachineOs - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<VirtualMachineOs> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VirtualMachineOs as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into VirtualMachineOs - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct VirtualMachinePerformance {
    /// CPU load %
    #[serde(rename = "cpuUsage")]
    pub cpu_usage: serde_json::Value,

    /// Memory usage in MB
    #[serde(rename = "memoryUsage")]
    pub memory_usage: serde_json::Value,

    /// Disk IO in KB/s
    #[serde(rename = "diskIo")]
    pub disk_io: serde_json::Value,

    /// Network IO in KB/s
    #[serde(rename = "networkIo")]
    pub network_io: serde_json::Value,

}

impl VirtualMachinePerformance {
    #[allow(clippy::new_without_default)]
    pub fn new(cpu_usage: serde_json::Value, memory_usage: serde_json::Value, disk_io: serde_json::Value, network_io: serde_json::Value, ) -> VirtualMachinePerformance {
        VirtualMachinePerformance {
            cpu_usage,
            memory_usage,
            disk_io,
            network_io,
        }
    }
}

/// Converts the VirtualMachinePerformance value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for VirtualMachinePerformance {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cpuUsage in query parameter serialization

            // Skipping memoryUsage in query parameter serialization

            // Skipping diskIo in query parameter serialization

            // Skipping networkIo in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a VirtualMachinePerformance value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for VirtualMachinePerformance {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cpu_usage: Vec<serde_json::Value>,
            pub memory_usage: Vec<serde_json::Value>,
            pub disk_io: Vec<serde_json::Value>,
            pub network_io: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing VirtualMachinePerformance".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cpuUsage" => intermediate_rep.cpu_usage.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "memoryUsage" => intermediate_rep.memory_usage.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "diskIo" => intermediate_rep.disk_io.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "networkIo" => intermediate_rep.network_io.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing VirtualMachinePerformance".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(VirtualMachinePerformance {
            cpu_usage: intermediate_rep.cpu_usage.into_iter().next().ok_or_else(|| "cpuUsage missing in VirtualMachinePerformance".to_string())?,
            memory_usage: intermediate_rep.memory_usage.into_iter().next().ok_or_else(|| "memoryUsage missing in VirtualMachinePerformance".to_string())?,
            disk_io: intermediate_rep.disk_io.into_iter().next().ok_or_else(|| "diskIo missing in VirtualMachinePerformance".to_string())?,
            network_io: intermediate_rep.network_io.into_iter().next().ok_or_else(|| "networkIo missing in VirtualMachinePerformance".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<VirtualMachinePerformance> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<VirtualMachinePerformance>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<VirtualMachinePerformance>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for VirtualMachinePerformance - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<VirtualMachinePerformance> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <VirtualMachinePerformance as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into VirtualMachinePerformance - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

