use proc_macro::TokenStream;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, Token,
};
use quote::quote;

struct Receiver {
    channel: Ident,
    msg: Ident,
    handler: Expr,
}

impl Parse for Receiver {
    fn parse(input: ParseStream) -> Result<Self> {
        let channel: Ident = input.parse()?;
        let in_parens;
        let _parens = parenthesized!(in_parens in input);
        let msg: Ident = in_parens.parse()?;
        let _arrow = input.parse::<Token![=>]>()?;
        let handler: Expr = input.parse()?;

        Ok(Receiver {
            channel,
            msg,
            handler,
        })
    }
}

fn parse_receivers(input: ParseStream) -> Result<Punctuated<Receiver, Token![,]>> {
    let receivers = input.parse_terminated(Receiver::parse)?;
    Ok(receivers)
}

#[proc_macro]
pub fn drain(input: TokenStream) -> TokenStream {
    let receivers = parse_macro_input!(input with parse_receivers);

    let mut ts = TokenStream::new();

    for Receiver { channel, msg, handler } in receivers {

        let expanded = quote!{
            println!("{:?}", #channel);
            #handler;
        };
        ts.extend(TokenStream::from(expanded));
    }

    ts
}
