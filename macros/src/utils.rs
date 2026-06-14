use core::fmt::Debug;

use proc_macro::TokenStream;

pub(crate) fn panic_with_ctx<T: Debug + ToString>(message: impl Into<String>, invalid_tokens: T) -> ! {
    let dbg_string = format!("{:#?}", invalid_tokens);
    panic!("{}\nInvalid token(s): {}\nDetailed token(s) information: {}", message.into(), invalid_tokens.to_string(), dbg_string);
}

pub(crate) fn debug_tokens(tokens: TokenStream) -> ! {
    panic!("debugging tokens:\n{:#?}", tokens);
}