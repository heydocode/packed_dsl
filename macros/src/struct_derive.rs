use std::{collections::HashMap, str::FromStr};

use proc_macro::{Group, TokenStream, TokenTree};

/// Must receive as input the TokenStream taken from
/// the delimiter TokenTree of the found enum.
pub(crate) fn parse_struct(tokens: TokenStream) -> HashMap<String, Vec<(u32, String)>> {
    let mut result: HashMap<String, Vec<(u32, String)>> = HashMap::new();

    let mut preceded_punct = true;
    let mut current_field: Option<String> = None;
    for variant in tokens.into_iter() {
        match variant {
            TokenTree::Group(g) => {
                if let Some(field) = &current_field {
                    result
                        .get_mut(field)
                        .expect("Expected valid value in current_field")
                        .append(&mut parse_group(g, 0));
                } else {
                    panic!(
                        "current_field must have been valid if encountered a group, so invalid struct, bug, or uncovered case has just happened."
                    );
                }
            }
            TokenTree::Punct(p) if p.to_string() == String::from(",") => {
                preceded_punct = true;
            },
            TokenTree::Ident(ident) => {
                if preceded_punct {
                    current_field = Some(ident.to_string());
                    result.insert(ident.to_string(), Vec::new());
                    preceded_punct = false;
                }
                else {
                    if let Some(field) = &current_field {
                    result
                        .get_mut(field)
                        .expect("Expected valid value in current_field")
                        .append(&mut vec![(0u32, ident.to_string())]);
                    }
                }
            },
            _ => ()
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

pub(crate) fn generate_impl(name: String, map: HashMap<String, Vec<(u32, String)>>) -> TokenStream {
    let enum_variant_num = map.len();
    let base_hash = format!(
        "packed_dsl::hash::hash(&{}u64.to_le_bytes(), size_of::<u64>())",
        enum_variant_num
    );

    let mut hashes = String::new();
    for (index, key) in map.clone().iter().enumerate() {
        // Here we hash the index and not the key to avoid
        // changing the hash as soon as the field name changes
        hashes += format!(
            "packed_dsl::hash::hash(&{}u64.to_le_bytes(), size_of::<u64>()),",
            index
        )
        .as_str();
        for val in key.1 {
            hashes +=
                format!("<{} as packed_dsl::proto::DslProto<'static>>::HASH,", val.1).as_str();
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
