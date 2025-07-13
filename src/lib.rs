extern crate serde;
extern crate serde_json;
extern crate serde_logger_derive;

use serde::{Serialize, Serializer};

pub fn serde_log<T: Serialize>(t: &T) {
    let string = serde_json::to_string(t).unwrap();
    println!("{}", string);
}

pub trait SerializesFor<T> {
    fn serialize_for<S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error>;
}

pub fn remote_serde_log<T1, T2>(t: &T2) where T1: SerializesFor<T2> {
    let mut bytes = Vec::new();
    let mut serialize = serde_json::ser::Serializer::new(&mut bytes);
    T1::serialize_for(t, &mut serialize).unwrap();
    println!("{}", String::from_utf8_lossy(&bytes).to_string());
}

pub use serde_logger_derive::SerializesFor;