use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, Token,
};

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

type Receivers = Punctuated<Receiver, Token![,]>;

fn parse_receivers(input: ParseStream) -> Result<Receivers> {
    input.parse_terminated(Receiver::parse)
}

#[proc_macro]
pub fn drain(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let receivers = parse_macro_input!(input with parse_receivers);

    let channels: Vec<&Ident> = receivers.iter().map(|r| &r.channel).collect();
    let channel_len = channels.len();

    let selectors = build_selectors(&channels);
    let op_matches = build_op_match(&receivers);

    let whole = quote! {{
        // We can just keep track of the number of remaining channels open,
        // since we remove each channel from the `Select` below as soon as
        // it errors once. (We could skip this entirely if `Select` had a len().)
        let mut channels_open: usize = #channel_len;

        let mut sel = crossbeam::channel::Select::new();
        #selectors

        // While any channels are open, keep receiving.
        while channels_open > 0 {
            let op = sel.select();
            match op.index() {
                #op_matches
                wut => unreachable!("Unexpected index {}", wut)
            }
        }
    }};

    whole.into()
}

fn build_selectors(channels: &[&Ident]) -> TokenStream {
    let mut selectors = TokenStream::new();
    for (i, channel) in channels.iter().enumerate() {
        selectors.extend(quote! {
            assert_eq!(sel.recv(&#channel), #i);
        })
    }
    selectors
}

fn build_op_match(receivers: &Receivers) -> TokenStream {
    let mut match_arms = TokenStream::new();
    for (
        i,
        Receiver {
            channel,
            msg,
            handler,
        },
    ) in receivers.iter().enumerate()
    {
        match_arms.extend(quote! {
            idx if idx == #i => {
                match op.recv(&#channel) {
                    Ok(#msg) => #handler,
                    Err(_) => {
                        // Indexes are stable; this doesn't shift remaining channels.
                        sel.remove(#i);
                        assert!(channels_open > 0);
                        channels_open -= 1;
                    }
                }
            },
        })
    }
    match_arms
}
