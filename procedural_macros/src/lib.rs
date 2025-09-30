use proc_macro::{Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

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
            // Remove the '<' punctuation, and add it to the output.
            token_stream.next();
            output.extend([TokenTree::Punct(Punct::new('<', Spacing::Alone))]);

            impl_variadic(&mut token_stream, &mut output);
        } else {
            output.extend([token_tree]);
        }
    }

    output
}

enum ImplVariadicStage {
    Generics,
}

fn finish_generic_position_variadic(name: String, span: Span, bounds: Vec<Ident>, output: &mut TokenStream) {
    for i in 0..8_u8 {
        let identifier = Ident::new(&format!("{name}{i}"), span);
        output.extend([TokenTree::Ident(identifier)]);

        let mut bounds = bounds.iter();

        if let Some(bound) = bounds.next() {
            output.extend([TokenTree::Punct(Punct::new(':', Spacing::Alone)), TokenTree::Ident(bound.clone())]);

            for bound in bounds {
                output.extend([TokenTree::Punct(Punct::new(
                    '+',
                    Spacing::Alone,
                )), TokenTree::Ident(bound.clone())]);
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
                    output.extend([TokenTree::Punct(punctuation)]);
                    return;
                }
                ':' => {
                    // Only extend the output with ':' if we aren't in a generic position variadic.
                    if generic_position_variadic.is_none() {
                        output.extend([TokenTree::Punct(punctuation)])
                    }
                }
                ',' => {
                    if let Some((name, span, bounds)) = generic_position_variadic.take() {
                        finish_generic_position_variadic(name, span, bounds, output);
                    } else {
                        // Only extend the output with ',' if we aren't finishing a generic position variadic.
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
                            .map(|name| (name.to_owned(), identifier.span(), None));
                    }
                    Some((_, _, Some(bounds))) => {
                        // If bounds is accessible, then we must be a bound.
                        bounds.push(identifier);
                    }
                    _ => {
                        // In all other situations the user has types invalid syntax similar to
                        // `VariadicBad BadBad`. Return and let the compiler correct them.
                        output.extend([TokenTree::Ident(identifier)]);
                        return;
                    }
                }
            }
            other => {
                output.extend([other]);
            }
        }
    }
}

fn variadic_recursive(token_stream: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    let mut impl_previous = false;
    let mut inside_impl_generics = false;
    let mut inside_impl_generic_variadic: Option<(String, Span, Option<Vec<Ident>>)> = None;

    for token_tree in token_stream {
        let mut found_impl = false;
        match token_tree {
            // TokenTree::Group(group) => {
            //     let new_inner = variadic_recursive(group.stream());
            //     let mut new_group = Group::new(group.delimiter(), new_inner);
            //     new_group.set_span(group.span());
            //     TokenTree::Group(new_group)
            // }
            TokenTree::Ident(ident) => {
                let ident_string = ident.to_string();

                found_impl = ident_string == "impl";

                if inside_impl_generics
                    && inside_impl_generic_variadic.is_none()
                    && let Some(name) = ident_string.strip_prefix("Variadic")
                {
                    inside_impl_generic_variadic = Some((name.to_owned(), ident.span(), None));
                } else if inside_impl_generics
                    && let Some((_, _, Some(bounds))) = &mut inside_impl_generic_variadic
                {
                    bounds.push(ident);
                } else {
                    output.extend([TokenTree::Ident(ident)]);
                }
            }
            TokenTree::Punct(punct) => {
                fn finish_impl_generic_variadic(
                    inside_impl_generic_variadic: &mut Option<(String, Span, Option<Vec<Ident>>)>,
                    output: &mut TokenStream,
                    punct: Punct,
                ) {
                    let Some((name, span, bounds)) = inside_impl_generic_variadic.take() else {
                        return;
                    };

                    if let Some(bounds) = bounds {
                        for i in 0..8_u8 {
                            let variadic_ident = Ident::new(&format!("{name}{i}"), span);
                            output.extend([
                                TokenTree::Ident(variadic_ident),
                                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                            ]);
                            for (index, bound) in bounds.iter().enumerate() {
                                if index != 0 {
                                    output.extend([TokenTree::Punct(Punct::new(
                                        '+',
                                        Spacing::Alone,
                                    ))]);
                                }
                                output.extend([TokenTree::Ident(bound.clone())]);
                            }
                            output.extend([TokenTree::Punct(Punct::new(',', punct.spacing()))]);
                        }
                    } else {
                        for i in 0..8_u8 {
                            let variadic_ident = Ident::new(&format!("{name}{i}"), span);
                            output.extend([
                                TokenTree::Ident(variadic_ident),
                                TokenTree::Punct(Punct::new(',', punct.spacing())),
                            ]);
                        }
                    }
                }
                if impl_previous && punct.as_char() == '<' {
                    inside_impl_generics = true;
                    output.extend([TokenTree::Punct(punct)]);
                } else if inside_impl_generics && punct.as_char() == '>' {
                    inside_impl_generics = false;
                    // Attempt to finish any remaining variadics.
                    finish_impl_generic_variadic(
                        &mut inside_impl_generic_variadic,
                        &mut output,
                        punct.clone(),
                    );
                    output.extend([TokenTree::Punct(punct)]);
                } else if let Some((_, _, bounds)) = &mut inside_impl_generic_variadic
                    && punct.as_char() == ':'
                {
                    *bounds = Some(vec![]);
                } else if inside_impl_generic_variadic.is_some() && punct.as_char() == ',' {
                    finish_impl_generic_variadic(
                        &mut inside_impl_generic_variadic,
                        &mut output,
                        punct,
                    );
                } else if matches!(inside_impl_generic_variadic, Some((_, _, Some(_))))
                    && punct.as_char() == '+'
                {
                } else {
                    output.extend([TokenTree::Punct(punct)]);
                }
            }
            other => output.extend([other]),
        };

        impl_previous = found_impl;
    }
    output
}

fn variadic_alter_ident(ident: Ident) -> Ident {
    ident
}
