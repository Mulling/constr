use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn constr(attr: TokenStream, target: TokenStream) -> TokenStream {
    let mut to_str_fn = parse_attr(attr);

    if target.is_empty() {
        return emmit_error("expected something".into());
    }

    // TODO: Parse target
    expand(target);

    to_str_fn.extend([TokenTree::Group(Group::new(
        Delimiter::Brace,
        [TokenTree::Literal(Literal::string("test"))]
            .into_iter()
            .collect(),
    ))]);

    to_str_fn
}

fn expand(tokens: TokenStream) {
    tokens.into_iter().for_each(|token| match token {
        TokenTree::Group(group) => {
            println!("group: {group}");
            expand(group.stream());
        }
        TokenTree::Ident(ident) => {
            println!("ident: {ident}");
        }
        TokenTree::Punct(punct) => {
            println!("punct: {punct}");
        }
        TokenTree::Literal(lit) => {
            println!("lit: {lit}");
        }
    })
}

fn parse_attr(tokens: TokenStream) -> TokenStream {
    if tokens.is_empty() {}

    let mut tokens = tokens.into_iter();

    let name = match tokens.next() {
        Some(token) => match token {
            TokenTree::Ident(name) => name,
            _ => todo!(),
        },
        None => todo!(),
    };

    let arg_type = match tokens.next() {
        Some(arg) => match arg {
            // XXX: Clean up
            TokenTree::Group(group) => match group.stream().into_iter().next().unwrap() {
                TokenTree::Ident(arg) => arg,
                _ => todo!(),
            },
            _ => todo!(),
        },
        None => todo!(),
    };

    if let Some(token) = tokens.next() {
        return emmit_error(format!("unexpected {token} after type {arg_type}"));
    }

    emmit_fn(name, arg_type)
}

fn emmit_fn(name: Ident, arg_type: Ident) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("fn", Span::call_site())),
        TokenTree::Ident(name),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [
                TokenTree::Ident(Ident::new("arg", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Ident(arg_type),
            ]
            .into_iter()
            .collect(),
        )),
        TokenTree::Punct(Punct::new('-', Spacing::Joint)),
        TokenTree::Punct(Punct::new('>', Spacing::Joint)),
        TokenTree::Punct(Punct::new('&', Spacing::Joint)),
        TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
        TokenTree::Ident(Ident::new("static", Span::call_site())),
        TokenTree::Ident(Ident::new("str", Span::call_site())),
    ]
    .into_iter()
    .collect()
}

fn emmit_error(err: String) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", Span::mixed_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Joint)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::string(&err))]
                .into_iter()
                .collect(),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Joint)),
    ]
    .into_iter()
    .collect()
}
