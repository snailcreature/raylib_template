use serde::{Deserialize, Serialize};

/// Serialise the given data into a [RON](ron) [String].
pub fn ron_serialise<E: Serialize>(data: E) -> String {
    ron::to_string(&data).unwrap()
}

/// Deserialise a [RON](ron) [String] into a struct.
pub fn ron_deserialise<'a, E: Deserialize<'a>>(data: &'a str) -> E {
    ron::from_str(data).unwrap()
}
