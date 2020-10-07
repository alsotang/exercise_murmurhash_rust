use bytes::Buf;

fn murmur_32_scramble(init_k: u32) -> u32 {
    let mut k = init_k;
    k = k.wrapping_mul(0xcc9e2d51);
    k = (k << 15) | (k >> 17);
    k = k.wrapping_mul(0x1b873593);

    k
}

pub fn murmurhash(key_init: &[u8], seed: u32) -> u32 {
    let mut key = key_init;
    let mut h = seed;
    let len = key.len();


    while key.remaining() >= 4 {
        let k = key.get_u32();
        h ^= murmur_32_scramble(k);
        h = (h << 13) | (h >> 19);
        h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let mut k: u32 = 0;
    while key.has_remaining() {
        k |= key.get_u8() as u32;
        k <<= 8;
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
