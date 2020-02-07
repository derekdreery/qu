extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    spanned::Spanned,
    Block, Ident, ItemFn, Type,
};

macro_rules! bail {
    ($msg:expr) => {
        return Err(syn::Error::new(Span::call_site(), $msg));
    };
    ($span:expr, $msg:expr) => {
        return Err(syn::Error::new($span, $msg));
    };
}

#[proc_macro_attribute]
pub fn ick(_: proc_macro::TokenStream, s: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let main: Main = match syn::parse(s) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    main.to_token_stream().into()
}

struct Main {
    opt_name: Ident,
    opt_type: Type,
    body: Box<Block>,
}

impl Parse for Main {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: ItemFn = Parse::parse(input)?;
        let inputs = &inner.sig.inputs;
        if inputs.len() != 1 {
            bail!(inputs.span(), "main function should have 1 argument");
        }
        let (opt_name, opt_type) = get_ident(inputs.first().unwrap())?;
        Ok(Main {
            opt_name,
            opt_type,
            body: inner.block,
        })
    }
}

impl ToTokens for Main {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let opt_name = &self.opt_name;
        let opt_type = &self.opt_type;
        let body = &self.body;
        tokens.extend(quote! {
            fn _main_inner(#opt_name: #opt_type) -> ::qu::ick_use::Result {
                #body
            }
            fn main() {
                let #opt_name: #opt_type = ::qu::ick_use::StructOpt::from_args();
                let log_level = match #opt_name.verbosity {
                    0 => ::qu::ick_use::log::LevelFilter::Error,
                    1 => ::qu::ick_use::log::LevelFilter::Warn,
                    2 => ::qu::ick_use::log::LevelFilter::Info,
                    3 => ::qu::ick_use::log::LevelFilter::Debug,
                    _ => ::qu::ick_use::log::LevelFilter::Trace,
                };
                ::qu::env_logger::builder()
                    .filter_level(log_level)
                    .init();
                if let Err(e) = _main_inner(#opt_name) {
                    ::qu::ick_use::log::error!("{:?}", e);
                    ::std::process::exit(1);
                }
            }
        });
    }
}

fn get_ident(input: &syn::FnArg) -> Result<(Ident, Type)> {
    use syn::{FnArg, Pat};
    Ok(match input {
        FnArg::Typed(v) => match &*v.pat {
            Pat::Ident(v_inner) => (v_inner.ident.clone(), (*v.ty).clone()),
            _ => bail!(v.pat.span(), "should be an ident (e.g. `opt`)"),
        },
        _ => bail!(input.span(), "argument should be in form `opt: Opt`)"),
    })
}
