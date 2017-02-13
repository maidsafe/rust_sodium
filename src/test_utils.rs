#[cfg(all(test, feature = "serde"))]
extern crate serde_json;
#[cfg(all(test, feature = "serde"))]
extern crate rmp_serde;
#[cfg(all(test, feature = "serde"))]
extern crate core;
#[cfg(all(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

// Encodes then decodes `value` using JSON
#[cfg(all(test, feature = "serde"))]
pub fn round_trip<T>(value: T)
    where T: Serialize + Deserialize + Eq + core::fmt::Debug
{
    let encoded_value = unwrap!(serde_json::to_string(&value));
    let decoded_value = unwrap!(serde_json::from_str(&encoded_value));
    assert_eq!(value, decoded_value);

    let mut buf = Vec::new();
    value
        .serialize(&mut rmp_serde::Serializer::new(&mut buf))
        .unwrap();
    let mut de = rmp_serde::Deserializer::new(&buf[..]);
    let decoded_value = Deserialize::deserialize(&mut de).unwrap();
    assert_eq!(value, decoded_value);
}


#[cfg(all(test, feature = "rustc-serialize", not(feature = "serde")))]
use rustc_serialize::{Decodable, Encodable, json};

// Encodes then decodes `value` using JSON
#[cfg(all(test, feature = "rustc-serialize", not(feature = "serde")))]
pub fn round_trip<T>(value: T)
    where T: Decodable + Encodable + Eq
{
    let encoded_value = unwrap!(json::encode(&value));
    let decoded_value = unwrap!(json::decode(&encoded_value));
    assert!(value == decoded_value);
}
