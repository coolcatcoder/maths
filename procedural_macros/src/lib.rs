use proc_macro::{Ident, TokenStream, TokenTree};

/// Any identifier found in the token stream will have LINE replaced with the line number.
#[proc_macro]
pub fn line_identifier(token_stream: TokenStream) -> TokenStream {
    TokenStream::from_iter(token_stream.into_iter().map(|token_tree| {
        match token_tree {
            TokenTree::Ident(ident) => {
                TokenTree::Ident(Ident::new(ident.to_string().replace("LINE", ident.span().line().to_string().as_str()).as_str(), ident.span()))
            }
            other => other,
        }
    }))
    // let TokenTree::Ident(name) = name.into_iter().next().unwrap() else {
    //     panic!();
    // };

    // format!("{}{}", name, name.span().line()).parse().unwrap()
}