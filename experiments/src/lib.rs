use packed_dsl::macros::DslProto;
use packed_dsl::proto::DslProto;

#[derive(DslProto)]
pub(crate) enum DerivedEnum {
    Variant1,
    Variant2,
    Variant3,
}

/// False test, this whole library "experimentations" is only
/// meant for development and experiments, hence its name.
#[test]
fn test() {
    panic!("{}", DerivedEnum::HASH);
}