use murmurhash::murmurhash;

#[test]
fn compute() {
    let result = murmurhash(b"aaaa", 97);
    assert_eq!(result, 3022694594);
}
