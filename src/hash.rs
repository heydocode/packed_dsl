/// Hashes idents using the FNV-1a hashing algorithm.
/// https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
/// 
/// Note that idents' lifetimes should be omitted from the input (as they
/// don't participate in serialization, nor they do in deserialization).
/// 
/// This function should never be executed at runtime, except when
/// generating DSL contents.
pub const fn hash_str(ident_body: &'static str) -> u64 {
    unimplemented!()
}
