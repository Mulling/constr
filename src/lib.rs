use proc_macro2::{Delimiter, Group, Literal, TokenStream, TokenTree};
use quote::quote;
use std::collections::HashMap;
use std::str::FromStr;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Paren,
    token::{self, Bracket},
    Ident, LitStr, Token,
};

macro_rules! skip {
    ($i:ident to $e:pat) => {
        skip!($i to $e if true)
    };
    ($i:ident to $e:pat if $c:expr) => {
        $i.step(|cursor| {
            let mut next = *cursor;
            while let Some((tt, xs)) = next.token_tree() {
                match &tt {
                    $e if $c => {
                        return Ok(((), xs));
                    }
                    _ => next = xs,
                }
            }
            Err(cursor.error("TODO:"))
        })
    };
    ($i:ident til $e:pat) => {
        skip!($i til $e if true)
    };
    ($i:ident til $e:pat if $c:expr) => {
        $i.step(|cursor| {
            let mut next = *cursor;
            while let Some((tt, xs)) = next.token_tree() {
                match &tt {
                    $e if $c => {
                        return Ok(((), next));
                    }
                    _ => next = xs,
                }
            }
            Err(cursor.error("TODO"))
        })
    };
}

//#[constr(signature(type) -> return]
//         ^         ^
//         |         |__type
//         |____________name
//
// TODO: access modifier!? (pub)
// TODO: return type
#[derive(Debug)]
struct Signature {
    name: Ident,
    r#type: Ident,
}

impl Parse for Signature {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let inner;
        let _: Paren = parenthesized!(inner in input);
        let r#type = inner.parse::<Ident>()?;

        if !inner.is_empty() {
            return Err(inner.error("only one type"));
        }

        Ok(Self { name, r#type })
    }
}

impl Signature {
    // TODO: This should be inside the module
    fn emmit(self) -> TokenStream {
        let name = self.name;
        let r#type = self.r#type;
        quote! {
            const fn #name(arg: #r#type) -> &'static str
        }
    }
}

// #[str = ""]
// const IDENT: type = ...;
#[derive(Debug)]
struct StrHelper {
    what: String,
    value: String,
    // r#type: String, // TODO: Check the type, Needed?
}

impl Parse for StrHelper {
    // TODO: Type check
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![#]>()?;

        let inner;
        let _: Bracket = bracketed!(inner in input);

        let to = inner.parse::<Ident>()?;
        // TODO: We might find other things that we have to ignore, such as cfg
        if to.to_string() != "str" {
            return Err(inner.error("TODO: expected str"));
        }

        inner.parse::<Token![=]>()?;
        let what = inner.parse::<LitStr>()?;

        // CLEAN: Might not be necessary
        if !inner.is_empty() {
            return Err(inner.error("expected ]"));
        }

        skip!(input to TokenTree::Ident(ident) if ident.to_string() == "const")?;
        input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let _type = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<Literal>()?;
        input.parse::<Token![;]>()?;

        Ok(StrHelper {
            what: what.value(),
            value: value.to_string(),
            // r#type: r#type.to_string(),
        })
    }
}

fn filter_helpers(tokens: TokenStream) -> TokenStream {
    let mut ret = Vec::<TokenTree>::new();
    let mut tokens = tokens.into_iter().peekable();

    // CLEANUP:
    'outer: while let Some(token) = tokens.next() {
        match &token {
            TokenTree::Punct(ref punct) if punct.as_char() == '#' => {
                if let Some(TokenTree::Group(group)) = tokens.peek() {
                    if group.delimiter() == Delimiter::Bracket {
                        match group.stream().into_iter().next() {
                            Some(TokenTree::Ident(ident)) if ident.to_string() == "str" => {
                                tokens.next();
                                continue 'outer;
                            }
                            _ => (),
                        }
                    }
                    ret.push(token);
                }
            }

            TokenTree::Group(group) => {
                ret.push(Group::new(group.delimiter(), filter_helpers(group.stream())).into())
            }
            _ => ret.push(token),
        }
    }

    ret.into_iter().collect()
}

#[derive(Debug)]
struct Constr {
    tokens: proc_macro::TokenStream,
    helpers: Vec<StrHelper>,
}

impl Parse for Constr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tokens: proc_macro::TokenStream = input.cursor().token_stream().clone().into();

        skip!(input to TokenTree::Ident(ident) if ident.to_string() == "mod")?;

        input.parse::<Ident>()?;

        let inner;
        let _: token::Brace = braced!(inner in input);

        skip!(inner til TokenTree::Punct(punct) if punct.as_char() == '#')?;

        let mut helpers = Vec::<StrHelper>::new();

        'do_while: loop {
            helpers.push(inner.parse::<StrHelper>()?);

            if inner.is_empty() {
                break 'do_while;
            }
        }

        Ok(Constr { tokens, helpers })
    }
}

impl Constr {
    // TODO: Account for duplicates
    fn emmit(self, sig: Signature) -> TokenStream {
        let mut map: HashMap<String, String> = HashMap::new();

        self.helpers.into_iter().for_each(|helper| {
            map.entry(helper.value).or_insert(helper.what);
        });

        let mut tokens = filter_helpers(self.tokens.into());
        let signature = sig.emmit();

        tokens.extend(signature);
        tokens.extend(Self::emmit_match(map));
        tokens
    }

    fn emmit_match(map: HashMap<String, String>) -> TokenStream {
        let inner = Self::emmit_match_inner(map);

        quote! {
            {
                match arg {
                    #inner
                    _ => unreachable!(),
                }
            }
        }
        .into()
    }

    fn emmit_match_inner(map: HashMap<String, String>) -> proc_macro2::TokenStream {
        // CLEANUP: Can we error out here?
        map.into_iter()
            .map(|(k, v)| {
                let num = Literal::from_str(&k).unwrap();

                quote!( #num => #v, )
            })
            .collect()
    }
}

#[proc_macro_attribute]
pub fn constr(
    attribute: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let constr = parse_macro_input!(tokens as Constr);

    let sig = parse_macro_input!(attribute as Signature);

    constr.emmit(sig).into()
}
