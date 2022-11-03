use std::hash::{BuildHasher, Hasher};

use mmh3_32::murmurhash3_x86_32;

pub struct Murmur3Hasher {
    seed: u32,
    bytes: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct Murmur3HashState {
    seed: u32,
}

impl Murmur3HashState {
    pub fn new() -> Murmur3HashState {
        return Murmur3HashState { seed: 0 };
    }

    pub fn with_seed(seed: u32) -> Murmur3HashState {
        return Murmur3HashState { seed };
    }
}

impl Hasher for Murmur3Hasher {
    fn finish(&self) -> u64 {
        return murmurhash3_x86_32(&self.bytes, self.seed) as u64;
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut copy = bytes.clone().to_vec();
        self.bytes.append(&mut copy);
    }
}

impl BuildHasher for Murmur3HashState {
    type Hasher = Murmur3Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        Murmur3Hasher {
            seed: self.seed,
            bytes: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::Murmur3HashState;
    use std::collections::hash_map::HashMap;

    #[test]
    fn use_in_hashmap() {
        let mut hashmap = HashMap::with_capacity_and_hasher(0, Murmur3HashState::new());
        hashmap.insert("one", 1);
        hashmap.insert("two", 2);

        assert_eq!(hashmap.len(), 2);

        assert_eq!(*hashmap.get("one").unwrap(), 1);
        assert_eq!(*hashmap.get("two").unwrap(), 2);
    }
}
