use std::old_io::BufReader;
use std::num::Int;

fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k *= 0xff51afd7ed558ccdu64;
    k ^= k >> 33;
    k *= 0xc4ceb9fe1a85ec53u64;
    k ^= k >> 33;

    return k;
}

pub fn murmurhash3_x64_128(bytes: &[u8], seed: u64) -> (u64, u64) {
    let c1 = 0x87c37b91114253d5u64;
    let c2 = 0x4cf5ad432745937fu64;
    let read_size = 16;

    let (mut h1, mut h2) = (seed, seed);

    let mut reader = BufReader::new(bytes);
    let mut remaining = bytes.len();

    while remaining >= read_size {
        let mut k1 = reader.read_le_u64().unwrap();
        let mut k2 = reader.read_le_u64().unwrap();

        k1 *= c1;
        k1 = k1.rotate_left(31);
        k1 *= c2;
        h1 ^= k1;

        h1 = h1.rotate_left(27);
        h1 += h2;
        h1 = h1 * 5 + 0x52dce729;

        k2 *= c2;
        k2 = k2.rotate_left(33);
        k2 *= c1;
        h2 ^= k2;

        h2 = h2.rotate_left(31);
        h2 += h1;
        h2 = h2 * 5 + 0x38495ab5;

        remaining -= read_size;
    }

    let tail = reader.read_to_end().unwrap();
    let (mut k1, mut k2) = (0u64, 0u64);

    if tail.len() == 15 { k2 ^= (tail[14] as u64) << 48; }
    if tail.len() >= 14 { k2 ^= (tail[13] as u64) << 40; }
    if tail.len() >= 13 { k2 ^= (tail[12] as u64) << 32; }
    if tail.len() >= 12 { k2 ^= (tail[11] as u64) << 24; }
    if tail.len() >= 11 { k2 ^= (tail[10] as u64) << 16; }
    if tail.len() >= 10 { k2 ^= (tail[ 9] as u64) <<  8; }
    if tail.len() >=  9 { k2 ^=  tail[ 8] as u64;
        k2 *= c2;
        k2 = k2.rotate_left(33);
        k2 *= c1;
        h2 ^= k2;
    }

    if tail.len() >= 8 { k1 ^= (tail[7] as u64) << 56; }
    if tail.len() >= 7 { k1 ^= (tail[6] as u64) << 48; }
    if tail.len() >= 6 { k1 ^= (tail[5] as u64) << 40; }
    if tail.len() >= 5 { k1 ^= (tail[4] as u64) << 32; }
    if tail.len() >= 4 { k1 ^= (tail[3] as u64) << 24; }
    if tail.len() >= 3 { k1 ^= (tail[2] as u64) << 16; }
    if tail.len() >= 2 { k1 ^= (tail[1] as u64) <<  8; }
    if tail.len() >= 1 { k1 ^=  tail[0] as u64;
        k1 *= c1;
        k1 = k1.rotate_left(31);
        k1 *= c2;
        h1 ^= k1;
    }

    h1 ^= bytes.len() as u64;
    h2 ^= bytes.len() as u64;

    h1 += h2;
    h2 += h1;

    h1 = fmix64(h1);
    h2 = fmix64(h2);

    h1 += h2;
    h2 += h1;

    return (h1, h2);
}

#[cfg(test)]
mod test {
    use super::murmurhash3_x64_128;

    #[test]
    fn test_empty_string() {
        assert!(murmurhash3_x64_128("".as_bytes(), 0) == (0, 0));
    }

    #[test]
    fn test_tail_lengths() {
        assert!(murmurhash3_x64_128("1".as_bytes(), 0)
            == (8213365047359667313, 10676604921780958775));
        assert!(murmurhash3_x64_128("12".as_bytes(), 0)
            == (5355690773644049813, 9855895140584599837));
        assert!(murmurhash3_x64_128("123".as_bytes(), 0)
            == (10978418110857903978, 4791445053355511657));
        assert!(murmurhash3_x64_128("1234".as_bytes(), 0)
            == (619023178690193332, 3755592904005385637));
        assert!(murmurhash3_x64_128("12345".as_bytes(), 0)
            == (2375712675693977547, 17382870096830835188));
        assert!(murmurhash3_x64_128("123456".as_bytes(), 0)
            == (16435832985690558678, 5882968373513761278));
        assert!(murmurhash3_x64_128("1234567".as_bytes(), 0)
            == (3232113351312417698, 4025181827808483669));
        assert!(murmurhash3_x64_128("12345678".as_bytes(), 0)
            == (4272337174398058908, 10464973996478965079));
        assert!(murmurhash3_x64_128("123456789".as_bytes(), 0)
            == (4360720697772133540, 11094893415607738629));
        assert!(murmurhash3_x64_128("123456789a".as_bytes(), 0)
            == (12594836289594257748, 2662019112679848245));
        assert!(murmurhash3_x64_128("123456789ab".as_bytes(), 0)
            == (6978636991469537545, 12243090730442643750));
        assert!(murmurhash3_x64_128("123456789abc".as_bytes(), 0)
            == (211890993682310078, 16480638721813329343));
        assert!(murmurhash3_x64_128("123456789abcd".as_bytes(), 0)
            == (12459781455342427559, 3193214493011213179));
        assert!(murmurhash3_x64_128("123456789abcde".as_bytes(), 0)
            == (12538342858731408721, 9820739847336455216));
        assert!(murmurhash3_x64_128("123456789abcdef".as_bytes(), 0)
            == (9165946068217512774, 2451472574052603025));
        assert!(murmurhash3_x64_128("123456789abcdef1".as_bytes(), 0)
            == (9259082041050667785, 12459473952842597282));
    }

    #[test]
    fn test_large_data() {
        assert!(murmurhash3_x64_128("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam at consequat massa. Cras eleifend pellentesque ex, at dignissim libero maximus ut. Sed eget nulla felis".as_bytes(), 0)
            == (9455322759164802692, 17863277201603478371));
    }
}
