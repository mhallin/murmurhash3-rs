mod mmh3_128;
mod mmh3_32;

mod hasher;

pub use mmh3_128::murmurhash3_x64_128;
pub use mmh3_32::murmurhash3_x86_32;

pub use hasher::Murmur3HashState;
