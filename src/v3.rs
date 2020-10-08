use bytes::Buf;

fn murmur_32_scramble(k: u32) -> u32 {
    let mut k2 = k;
    k2 = k2.wrapping_mul(0xcc9e2d51);
    k2 = (k2 << 15) | (k2 >> 17);
    k2 = k2.wrapping_mul(0x1b873593);

    k2
}

pub fn murmur32(key: &[u8], seed: u32) -> u32 {
    let mut buf = key;
    let mut h = seed;
    let len = buf.len();

    while buf.remaining() >= 4 {
        let k = buf.get_u32_le();

        h ^= murmur_32_scramble(k);
        h = (h << 13) | (h >> 19);

        h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let mut k: u32 = 0;
    for n in buf.iter().rev() {
        k <<= 8;
        k |= *n as u32;
    }

    h ^= murmur_32_scramble(k);

    /* Finalize. */
    h ^= len as u32;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;

    h
}
