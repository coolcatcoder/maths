use proc_macro::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};

/// Any identifier found in the token stream will have LINE replaced with the
/// line number.
#[proc_macro]
pub fn line_identifier(token_stream: TokenStream) -> TokenStream {
    TokenStream::from_iter(token_stream.into_iter().map(|token_tree| {
        match token_tree {
            TokenTree::Ident(ident) => TokenTree::Ident(Ident::new(
                ident
                    .to_string()
                    .replace("LINE", ident.span().line().to_string().as_str())
                    .as_str(),
                ident.span(),
            )),
            other => other,
        }
    }))
}

/// Any identifier found in the token stream will have LINE replaced with the
/// line number.
#[proc_macro_attribute]
pub fn variadic(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();

    let mut token_stream = token_stream.into_iter().peekable();

    while let Some(token_tree) = token_stream.next() {
        if let TokenTree::Ident(ident) = &token_tree
            && ident.to_string().as_str() == "impl"
            && let Some(TokenTree::Punct(punct)) = token_stream.peek()
            && punct.as_char() == '<'
        {
            // Remove the '<' punctuation, and add the 'impl' and it to the output.
            token_stream.next();
            output.extend([
                TokenTree::Ident(ident.clone()),
                TokenTree::Punct(Punct::new('<', Spacing::Alone)),
            ]);

            impl_variadic(&mut token_stream, &mut output);
        } else {
            output.extend([token_tree]);
        }
    }

    output
}

fn finish_generic_position_variadic(
    name: String,
    span: Span,
    bounds: Vec<Ident>,
    output: &mut TokenStream,
) {
    for i in 0..8_u8 {
        let identifier = Ident::new(&format!("{name}{i}"), span);
        output.extend([TokenTree::Ident(identifier)]);

        let mut bounds = bounds.iter();

        if let Some(bound) = bounds.next() {
            output.extend([
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(bound.clone()),
            ]);

            for bound in bounds {
                output.extend([
                    TokenTree::Punct(Punct::new('+', Spacing::Alone)),
                    TokenTree::Ident(bound.clone()),
                ]);
            }
        }

        output.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
    }
}

fn impl_variadic(token_stream: &mut impl Iterator<Item = TokenTree>, output: &mut TokenStream) {
    // VariadicSomething: Bound1 + Bound2
    let mut generic_position_variadic: Option<(String, Span, Vec<Ident>)> = None;

    for token_tree in token_stream {
        match token_tree {
            TokenTree::Punct(punctuation) => match punctuation.as_char() {
                '>' => {
                    if let Some((name, span, bounds)) = generic_position_variadic {
                        finish_generic_position_variadic(name, span, bounds, output);
                    }
                    output.extend([TokenTree::Punct(punctuation)]);
                    return;
                }
                ':' | '+' => {
                    // Only extend the output with ':' or '+' if we aren't in a generic position
                    // variadic.
                    if generic_position_variadic.is_none() {
                        output.extend([TokenTree::Punct(punctuation)])
                    }
                }
                ',' => {
                    if let Some((name, span, bounds)) = generic_position_variadic.take() {
                        finish_generic_position_variadic(name, span, bounds, output);
                    } else {
                        // Only extend the output with ',' if we aren't finishing a generic position
                        // variadic.
                        output.extend([TokenTree::Punct(punctuation)])
                    }
                }
                _ => output.extend([TokenTree::Punct(punctuation)]),
            },
            TokenTree::Ident(identifier) => {
                match &mut generic_position_variadic {
                    None => {
                        // If the identifier starts with "Variadic" then we start a generic position
                        // variadic.
                        generic_position_variadic = identifier
                            .to_string()
                            .strip_prefix("Variadic")
                            .map(|name| (name.to_owned(), identifier.span(), vec![]));

                        if generic_position_variadic.is_none() {
                            output.extend([TokenTree::Ident(identifier)]);
                        }
                    }
                    Some((_, _, bounds)) => {
                        // If bounds is accessible, then we must be a bound.
                        bounds.push(identifier);
                    }
                }
            }
            other => {
                output.extend([other]);
            }
        }
    }
}
