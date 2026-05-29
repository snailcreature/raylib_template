/// Calculates *`17x`*
pub fn hash_1d(x: u32) -> u32 {
    (x << 4) + x
}

/// Calculates *`17x + 31y`*
pub fn hash_2d(x: u32, y: u32) -> u32 {
    hash_1d(x) + ((y << 5) - y)
}

/// Calculates *`17x + 31y + 127z`*
pub fn hash_3d(x: u32, y: u32, z: u32) -> u32 {
    hash_2d(x, y) + ((z << 7) - z)
}
