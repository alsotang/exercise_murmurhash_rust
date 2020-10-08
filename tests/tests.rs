#[allow(clippy::string_lit_as_bytes)]
#[test]
fn v3() {
    let hash = murmurhash::v3::murmur32(b"aaaa", 97);
    assert_eq!(hash, 3022694594);

    let hash = murmurhash::v3::murmur32(b"abcd", 97);
    assert_eq!(hash, 3733657357);

    let hash = murmurhash::v3::murmur32("中国".as_bytes(), 97);
    assert_eq!(hash, 3216982393);

    let hash = murmurhash::v3::murmur32("hello, world!".as_bytes(), 1);
    assert_eq!(hash, 1080018637);

    let hash = murmurhash::v3::murmur32("hello, world!".as_bytes(), 283);
    assert_eq!(hash, 3198928967);

    let hash = murmurhash::v3::murmur32("abcdefghijkl".as_bytes(), 23);
    assert_eq!(hash, 2275871593);

    let hash = murmurhash::v3::murmur32("brendan ashworth".as_bytes(), 100);
    assert_eq!(hash, 2444978076);

    let hash = murmurhash::v3::murmur32("brendan ashworth".as_bytes(), 0);
    assert_eq!(hash, 3210849834);
}
