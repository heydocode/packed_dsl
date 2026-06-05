use proc_macro::{TokenStream, TokenTree};

mod enum_derive;
mod struct_derive;

use crate::enum_derive::{generate_impl, parse_enum};

#[proc_macro_derive(DslProto)]
pub fn dslproto_derive(tokens: TokenStream) -> TokenStream {
    // dbg_tokens(tokens.clone());
    let mut iter = tokens.into_iter();

    while let Some(token) = iter.next() {
        match token {
            TokenTree::Ident(ident) if ident.to_string() == String::from("enum") => {
                let enum_name: String = iter.next().expect("Expected token after \"enum\"").to_string();
                let enum_map = if let Some(TokenTree::Group(g)) = iter.next() {
                    parse_enum(g.stream())
                } else {
                    panic!("The enum name (\"{}\") must be followed by tokens", enum_name);
                };

                return generate_impl(enum_name, enum_map);
                
                // panic!("{:#?}\n\n{:#?}", enum_name, enum_map);
            },
            _ => ()
        }
    }
    
    TokenStream::new()
}
