use serde::{Deserialize, Serialize};

pub fn ron_serialise<E: Serialize>(data: E) -> String {
    ron::to_string(&data).unwrap()
}

pub fn ron_deserialise<'a, E: Deserialize<'a>>(data: &'a str) -> E {
    ron::from_str(data).unwrap()
}
