use std::old_io::BufReader;
use std::num::Int;

fn fmix32(mut h: u32) -> u32 {
    h ^= h >> 16;
    h *= 0x85ebca6b;
    h ^= h >> 13;
    h *= 0xc2b2ae35;
    h ^= h >> 16;

    return h;
}

pub fn murmurhash3_x86_32(bytes: &[u8], seed: u32) -> u32 {
    let c1 = 0xcc9e2d51u32;
    let c2 = 0x1b873593u32;
    let read_size = 4;

    let mut h1 = seed;

    let mut reader = BufReader::new(bytes);
    let mut remaining = bytes.len();

    while remaining >= read_size {
        let mut k1 = reader.read_le_u32().unwrap();

        k1 *= c1;
        k1 = k1.rotate_left(15);
        k1 *= c2;

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1 * 5 + 0xe6546b64;

        remaining -= read_size;
    }

    let tail = reader.read_to_end().unwrap();
    let mut k1 = 0u32;

    if tail.len() == 3 { k1 ^= (tail[2] as u32) << 16; }
    if tail.len() >= 2 { k1 ^= (tail[1] as u32) << 8; }
    if tail.len() >= 1 { k1 ^=  tail[0] as u32;
        k1 *= c1;
        k1 = k1.rotate_left(15);
        k1 *= c2;
        h1 ^= k1;
    }

    h1 ^= bytes.len() as u32;
    h1 = fmix32(h1);

    return h1;
}

#[cfg(test)]
mod test {
    use super::murmurhash3_x86_32;

    #[test]
    fn test_empty_string() {
        assert!(murmurhash3_x86_32("".as_bytes(), 0) == 0);
    }

    #[test]
    fn test_tail_lengths() {
        assert!(murmurhash3_x86_32("1".as_bytes(), 0)
            == 2484513939);
        assert!(murmurhash3_x86_32("12".as_bytes(), 0)
            == 4191350549);
        assert!(murmurhash3_x86_32("123".as_bytes(), 0)
            == 2662625771);
        assert!(murmurhash3_x86_32("1234".as_bytes(), 0)
            == 1914461635);
    }

    #[test]
    fn test_large_data() {
        assert!(murmurhash3_x86_32("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam at consequat massa. Cras eleifend pellentesque ex, at dignissim libero maximus ut. Sed eget nulla felis".as_bytes(), 0)
            == 1004899618);
    }
}
