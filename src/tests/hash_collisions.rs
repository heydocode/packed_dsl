use packed_dsl_macros::DslProto;
use crate::proto::DslProto;
use crate as packed_dsl;

#[derive(DslProto)]
enum A {
    One(u8, u16, u8, u16, u8, u16),
    Two(u16, u8, u16, u8, u16, u8),
}


#[derive(DslProto)]
enum B {
    OneReversed(u16, u8, u16, u8, u16, u8),
    TwoReversed(u8, u16, u8, u16, u8, u16),
}

#[test]
fn test_hash_collisions() {
    // TODO! Implement extensive fuzzing, using random (but deterministic) methods,
    // and ones that are effective against FNV-1a (to detect matches that a human wouldn't
    // accidentally create).
    assert!(A::HASH != B::HASH);
}