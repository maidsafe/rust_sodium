#[cfg(all(test, feature = "serde"))]
extern crate serde_json;
#[cfg(all(test, feature = "serde"))]
extern crate core;
#[cfg(all(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

// Encodes then decodes `value` using JSON
#[cfg(all(test, feature = "serde"))]
pub fn round_trip<T>(value: T)
    where T: Serialize + Deserialize + Eq + core::fmt::Debug
{
    let encoded_value = serde_json::to_string(&value).unwrap();
    let decoded_value = serde_json::from_str(&encoded_value).unwrap();
    assert_eq!(value, decoded_value);
}


#[cfg(all(test, feature = "rustc-serialize", not(feature = "serde")))]
use rustc_serialize::{Decodable, Encodable, json};

// Encodes then decodes `value` using JSON
#[cfg(all(test, feature = "rustc-serialize", not(feature = "serde")))]
pub fn round_trip<T>(value: T)
    where T: Decodable + Encodable + Eq
{
    let encoded_value = json::encode(&value).unwrap();
    let decoded_value = json::decode(&encoded_value).unwrap();
    assert!(value == decoded_value);
}
