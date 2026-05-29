pub mod hashing;
pub mod rng;

pub mod prelude {
    pub use crate::hashing::hash_2d;
    pub use crate::rng::Rng;
}

#[cfg(test)]
mod tests {}
