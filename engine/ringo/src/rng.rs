use std::time::SystemTime;

use crate::hashing::hash_3d;

/// Holds the seed value for pseudorandom number generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seed(u32);

impl Seed {
    /// Rotates the seed to a new value.
    pub fn rotate(&mut self) -> &Self {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self
    }

    /// Mutates current value by hashing it against a value.
    pub fn mutate(&mut self, mutagen: u32) -> &Self {
        self.0 = hash_3d(0, self.0, mutagen);
        self
    }

    /// Get the current value.
    pub fn get(&self) -> u32 {
        self.0
    }

    /// Copy the current value.
    pub fn copy(&self) -> Self {
        Seed(self.0)
    }

    /// Copy the value and rotate it.
    pub fn copy_and_rotate(&self) -> Self {
        *Seed(self.0).rotate()
    }
}

/// Allows the generation of pseudorandom numbers for the type `Unit`.
pub trait RandRange<Unit> {
    /// Generate a random value between `min` and `max`.
    fn rand_range(&mut self, min: Unit, max: Unit) -> Unit;
}

/// Generates pseudorandom numbers based on a stored seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rng {
    orginal_seed: Seed,
    seed: Seed,
}

impl Rng {
    /// Create a new generator from an explicit number.
    pub fn new(seed: u32) -> Self {
        Self {
            seed: Seed(seed),
            orginal_seed: Seed(seed),
        }
    }

    /// Create a new generator from an existing seed.
    pub fn from_seed(seed: Seed) -> Self {
        Self {
            seed,
            orginal_seed: seed,
        }
    }

    /// Create a new generator using the current `SystemTime` as a seed.
    ///
    /// Gets the time in seconds (`f32`) since the `SystemTime::UNIX_EPOCH` and uses
    /// `f32::to_bits()` to create a `u32`.
    ///
    /// Panics if the duration since `SystemTime::UNIX_EPOCH` is negative.
    pub fn from_now() -> Self {
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs_f32().to_bits(),
            Err(err) => panic!("{err:?}"),
        };

        Self {
            seed: Seed(now),
            orginal_seed: Seed(now),
        }
    }

    /// Generate a pseudorandom floating point number between 0 and 1.
    pub fn rand_f32(&mut self) -> f32 {
        self.seed.rotate();
        let tmp = self.seed.get() >> 9 | 1_065_353_216;
        f32::from_bits(tmp)
    }

    /// Get the original seed of this generator.
    pub fn get_seed(&self) -> Seed {
        self.orginal_seed
    }

    /// Reset the seed to its original value.
    pub fn reset(&mut self) -> &Self {
        self.seed = self.orginal_seed;
        self
    }

    /// Create a new distinct generator from this one.
    ///
    /// Rotates and then mutates the new generator's seed based on this generator's original seed,
    /// which will result in duplicate child-generators. Use `Rng::spawn_mutated` to create distinct
    /// new generators.
    pub fn spawn(&mut self) -> Self {
        let new_seed = *self
            .orginal_seed
            .copy_and_rotate()
            .mutate(self.orginal_seed.get());
        Rng {
            orginal_seed: new_seed,
            seed: new_seed,
        }
    }

    /// Create a new mutated generator based on the provided mutagen and this generator's original
    /// seed.
    pub fn spawn_mutated(&mut self, mutagen: u32) -> Self {
        let new_seed = *self.orginal_seed.copy().mutate(mutagen);
        Rng {
            orginal_seed: new_seed,
            seed: new_seed,
        }
    }

    /// Create a copy of this generator in its current state.
    pub fn copy(&self) -> Self {
        Rng {
            orginal_seed: self.orginal_seed,
            seed: self.seed,
        }
    }

    /// Create a copy of this generator and reset it to its initial state.
    pub fn copy_and_reset(&self) -> Self {
        *self.copy().reset()
    }
}

impl RandRange<f32> for Rng {
    fn rand_range(&mut self, min: f32, max: f32) -> f32 {
        self.rand_f32() * (max - min) + min
    }
}

impl RandRange<u32> for Rng {
    fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        let scalar: f32 = (max - min) as f32;
        let rand = self.rand_f32();
        (rand * scalar) as u32 + min
    }
}

impl RandRange<usize> for Rng {
    fn rand_range(&mut self, min: usize, max: usize) -> usize {
        let scalar: f32 = (max - min) as f32;
        let rand = self.rand_f32();
        (rand * scalar) as usize + min
    }
}
