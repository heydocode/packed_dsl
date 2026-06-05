use std::str::FromStr;

use proc_macro::{Group, TokenStream, TokenTree};

/// Must receive as input the TokenStream taken from
/// the delimiter TokenTree of the found enum.
pub(crate) fn parse_enum(tokens: TokenStream) -> Vec<(String, Vec<(u32, String)>)> {
    let mut result: Vec<(String, Vec<(u32, String)>)> = Vec::new();

    let mut current_index: Option<usize> = None;
    for variant in tokens.into_iter() {
        match variant {
            TokenTree::Literal(_) => panic!("In an enum, there shouldn't be any literals"),
            TokenTree::Group(g) => {
                if let Some(idx) = current_index {
                    result
                        .get_mut(idx)
                        .expect("Expected valid index in current_index")
                        .1
                        .append(&mut parse_group(g, 0));
                } else {
                    panic!(
                        "current_index must have been valid if encountered a group, so invalid enum, bug, or uncovered case has just happened."
                    );
                }
            }
            TokenTree::Punct(_) => (),
            TokenTree::Ident(ident) => {
                if let Some(idx) = current_index {
                    current_index = Some(idx + 1);
                } else {
                    current_index = Some(0)
                }
                result.push((ident.to_string(), Vec::new()));
            }
        }
    }
    result
}

fn parse_group(group: Group, mut par_off: u32) -> Vec<(u32, String)> {
    let mut res = Vec::new();

    let par = par_off.clone();

    for token in group.stream() {
        match token {
            TokenTree::Literal(_) => panic!("Unexpected literal in an enum definition"),
            TokenTree::Punct(_) => (),
            TokenTree::Ident(ident) => {
                res.push((par, ident.to_string()));
            }
            TokenTree::Group(g) => {
                par_off += 1;
                res.append(&mut parse_group(g, par_off))
            }
        }
    }

    res
}

pub(crate) fn generate_impl(name: String, map: Vec<(String, Vec<(u32, String)>)>) -> TokenStream {
    let enum_variant_num = map.len();
    let base_hash = format!("packed_dsl::hash::hash(&{}u64.to_le_bytes(), 1)", enum_variant_num);
    
    let mut hashes = String::new();
    for key in map.clone() {
        for val in key.1 {
            hashes += format!("<{} as packed_dsl::proto::DslProto<'static>>::HASH,", val.1).as_str();
        }
    }
    
    let hash_block = format!(
        "packed_dsl::hash::rehash_with_n_hashes({}, &[{}])",
        base_hash, hashes
    );

    // let mut _serialize_contents = todo!("");
    // let mut _deserialize_contents = todo!("");

    let output = format!("
        impl<'a> DslProto<'a> for {} {{
            type Error = packed_dsl::no_std_io2::io::Error;

            const HASH: u64 = {};

            fn serialize<W: packed_dsl::bitstream_io::BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error> {{
                Ok(())
            }}

            fn deserialize<R: packed_dsl::bitstream_io::BitRead + ?Sized>(r: &mut R, buffer: &'a mut [u8; 255]) -> Result<Self, Self::Error> {{
                panic!(\"\");
            }}
        }}
    ", name, hash_block);

    TokenStream::from_str(output.as_str())
        .expect("Expected successful generated impl to parsed Rust code convertion")
}
