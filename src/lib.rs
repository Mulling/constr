use std::iter::Peekable;

use proc_macro::{
    token_stream::IntoIter, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};

#[proc_macro_attribute]
pub fn constr(attribute: TokenStream, tokens: TokenStream) -> TokenStream {
    let constr = emmit_constr(attribute);

    let (head, body) = parse_module_parts(tokens.clone());

    println!("body = {body}");

    parse_module_body(body);

    tokens
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

fn attribute_to_signature(attribute: TokenStream) -> TokenStream {
    if attribute.is_empty() {
        todo!("error out: attribute list is empty");
    }

    let mut tokens = attribute.into_iter();

    let fn_name = match tokens.next() {
        Some(TokenTree::Ident(name)) => name,
        Some(_) => todo!("error out: expected indent, got somethin else"),
        _ => unreachable!(),
    };

    let group = match tokens.next() {
        Some(TokenTree::Group(group)) => group,
        Some(_) => todo!("error out: expeted a group, got something else"),
        None => todo!("support for type inference"),
    };

    match group.delimiter() {
        Delimiter::Parenthesis => (),
        _ => todo!("error out: should be invoked with (...)"),
    }

    let mut group_tokens = group.stream().into_iter();

    let arg_type = match group_tokens.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(_) => todo!("error out: expeted ident"),
        None => todo!("error out: empty group"),
    };

    if group_tokens.next().is_some() {
        todo!("error out: unexpected token after type");
    }

    if tokens.next().is_some() {
        todo!("error out: unexpected token after type signature");
    }

    emmit_signature(fn_name, arg_type)
}

fn add_body(mut signature: TokenStream) -> TokenStream {
    signature.extend([TokenTree::Group(Group::new(
        Delimiter::Brace,
        [TokenTree::Literal(Literal::string("test"))]
            .into_iter()
            .collect(),
    ))]);

    signature
}

fn emmit_signature(name: Ident, arg_type: Ident) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("pub", Span::call_site())),
        TokenTree::Ident(Ident::new("const", Span::call_site())),
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

fn emmit_constr(attribute: TokenStream) -> TokenStream {
    let signagure = attribute_to_signature(attribute);

    add_body(signagure)
}

fn parse_module_parts(tokens: TokenStream) -> (TokenStream, Group) {
    let mut head: Vec<TokenTree> = Vec::new();

    let mut tokens = tokens.into_iter();
    // XXX: Cleanup
    let mut in_mudule = false;
    'out: while let Some(token) = tokens.next() {
        match token {
            TokenTree::Ident(ref ident) => {
                let bail = in_mudule;
                in_mudule = ident.to_string() == "mod";

                head.push(token);

                if bail {
                    break 'out;
                }
            }
            _ => head.push(token),
        }
    }

    // The poor man's split
    let body: TokenStream = tokens.collect();

    if let Some(TokenTree::Group(group)) = body.into_iter().next() {
        (head.into_iter().collect(), group)
    } else {
        todo!("emmit error: expected a group")
    }
}

fn parse_module_body(body: Group) {
    // TODO: Check if the body is empty
    let mut expanded: Vec<TokenTree> = Vec::new();

    let mut tokens = body.stream().into_iter().peekable();

    // XXX: Cleanup
    while let Some(token) = tokens.next() {
        if let TokenTree::Punct(ref punct) = token {
            if punct.to_string() == "#" {
                let a = try_expand_to(&mut tokens);
                println!("{a:?}");
            } else {
                expanded.push(token)
            }
        } else {
            expanded.push(token);
        }
    }

    println!("expanded = {expanded:?}");
}

/// Expands a given constant of given type and value, advancing the iterator past the end of the
/// expression. Returns both the string and expression.
fn try_expand_to(tokens: &mut Peekable<IntoIter>) -> Option<String> {
    // TODO: Result<Option<(String, TokenStream)>, TokenStream>
    // XXX: Cleanup
    let binding = tokens.next();
    let group = if let Some(TokenTree::Group(ref group)) = binding {
        group
    } else {
        todo!("error out: expected proc macro group");
    };

    if group.delimiter() != Delimiter::Bracket {
        todo!("error out: expected [...]")
    }

    let mut tokens_in_group = group.stream().into_iter();

    if let Some(TokenTree::Ident(ident)) = tokens_in_group.next() {
        if ident.to_string() != "to" {
            return None;
        }
    } else {
        return None; // Bail if not ours
    }

    let string = if let Some(TokenTree::Group(group)) = tokens_in_group.next() {
        if group.delimiter() != Delimiter::Parenthesis {
            todo!("error out: expected (...)")
        } else {
            let string = if let Some(TokenTree::Literal(lit)) = group.stream().into_iter().next() {
                lit.to_string()
            } else {
                todo!("error out: expeted literal");
            };

            // TODO: Check remaning

            string
        }
    } else {
        todo!("error out: expected (...)")
    };

    let a = tokens.next();
    println!("{a:?}");

    Some(string)
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
