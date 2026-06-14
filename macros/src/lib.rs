use proc_macro::{TokenStream, TokenTree};

mod enum_derive;
mod struct_derive;
mod utils;

use crate::utils::{debug_tokens, panic_with_ctx};

#[proc_macro_derive(DslProto)]
pub fn dslproto_derive(tokens: TokenStream) -> TokenStream {
    // debug_tokens(tokens);
    let mut iter = tokens.into_iter();

    while let Some(token) = iter.next() {
        match token {
            TokenTree::Ident(ident) if ident.to_string() == String::from("enum") => {
                let enum_name: String = iter.next().expect("Expected token after \"enum\"").to_string();
                let next = iter.next();
                let enum_map = if let Some(TokenTree::Group(g)) = next {
                    enum_derive::parse_enum(g.stream())
                } else if let Some(TokenTree::Punct(p)) = next {
                    panic_with_ctx("packed_dsl doesn't support generics for static analysis reasons.", p);
                } else if let Some(n) = next {
                    panic_with_ctx(format!("Unexpected parsing error: the enum name (\"{}\") should have been followed by the enum body.", enum_name), n);
                } else {
                    panic!("Unexpected parsing error: no tokens after the enum name, this is not valid Rust.");
                };

                return enum_derive::generate_impl(enum_name, enum_map);
                
                // panic!("{:#?}\n\n{:#?}", enum_name, enum_map);
            },
            TokenTree::Ident(ident) if ident.to_string() == String::from("struct") => {
                let struct_name: String = iter.next().expect("Expected token after \"struct\"").to_string();
                let next = iter.next();
                let struct_map = if let Some(TokenTree::Group(g)) = next {
                    struct_derive::parse_struct(g.stream())
                } else if let Some(TokenTree::Punct(p)) = next {
                    panic_with_ctx("packed_dsl doesn't support generics for static analysis reasons.", p);
                } else if let Some(n) = next {
                    panic_with_ctx(format!("Unexpected parsing error: the struct name (\"{}\") should have been followed by the struct body.", struct_name), n);
                } else {
                    panic!("Unexpected parsing error: no tokens after the struct name, this is not valid Rust.");
                };

                return struct_derive::generate_impl(struct_name, struct_map);
                
                // panic!("{:#?}\n\n{:#?}", enum_name, enum_map);
            },
            _ => ()
        }
    }
    
    TokenStream::new()
}
