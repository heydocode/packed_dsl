use packed_dsl::dsl_string::DslString;
use packed_dsl::macros::DslProto;
use packed_dsl::proto::DslProto;

// #[derive(DslProto)]
// enum DerivedEnum {
//     Variant1,
//     Variant2(u64, u32),
//     Variant3(DslString),
// }

#[derive(DslProto)]
struct Abiba {
    a: u8,
    b: (u16, u8, u128)
}

/// False test, this whole library "experimentations" is only
/// meant for development and experiments, hence its name.
#[test]
fn test() {
    // panic!("{}", DerivedEnum::HASH);
}