extern crate proc_macro;

use log::LevelFilter;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    spanned::Spanned,
    Block, Ident, ItemFn, Token, Type,
};

macro_rules! bail {
    ($msg:expr) => {
        return Err(syn::Error::new(Span::call_site(), $msg));
    };
    ($span:expr, $msg:expr) => {
        return Err(syn::Error::new($span, $msg));
    };
}

macro_rules! compile_error {
    ($inner:expr) => {
        match $inner {
            Ok(v) => v,
            Err(e) => return e.into_compile_error().into(),
        }
    };
}

#[proc_macro_attribute]
pub fn ick(
    metadata: proc_macro::TokenStream,
    s: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let main = syn::parse_macro_input!(s as Main);
    let default_log_level = compile_error!(parse_log_level(metadata));
    let quick = Quick {
        main,
        default_log_level,
    };
    quote!(#quick).into()
}

fn parse_log_level(metadata: proc_macro::TokenStream) -> Result<LevelFilter> {
    struct ParseLevelFilter(Option<LevelFilter>);

    impl Parse for ParseLevelFilter {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.is_empty() {
                return Ok(ParseLevelFilter(None));
            }
            let label: Ident = input.parse()?;
            if label != "default_log_level" {
                bail!(label.span(), "expected `default_log_level`");
            }
            let _: Token![=] = input.parse()?;
            let level: Ident = input.parse()?;
            let level = if level == "off" {
                LevelFilter::Off
            } else if level == "error" {
                LevelFilter::Error
            } else if level == "warn" {
                LevelFilter::Warn
            } else if level == "info" {
                LevelFilter::Info
            } else if level == "debug" {
                LevelFilter::Debug
            } else if level == "trace" {
                LevelFilter::Trace
            } else {
                bail!(
                    level.span(),
                    "expected one of `off`, `error`, `warn`, `info`, `debug`, `trace`"
                )
            };
            Ok(ParseLevelFilter(Some(level)))
        }
    }
    // The default default is info
    syn::parse(metadata).map(|v: ParseLevelFilter| v.0.unwrap_or(LevelFilter::Info))
}

struct Quick {
    main: Main,
    default_log_level: LevelFilter,
}

impl Quick {
    fn log_level_as_int(&self) -> i8 {
        match self.default_log_level {
            LevelFilter::Off => 0,
            LevelFilter::Error => 1,
            LevelFilter::Warn => 2,
            LevelFilter::Info => 3,
            LevelFilter::Debug => 4,
            LevelFilter::Trace => 5,
        }
    }
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

impl ToTokens for Quick {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let opt_name = &self.main.opt_name;
        let opt_type = &self.main.opt_type;
        let body = &self.main.body;
        let default_log = self.log_level_as_int();
        tokens.extend(quote! {
            #[derive(StructOpt)]
            struct __wrapping_Opt {
                #[structopt(flatten)]
                #opt_name: #opt_type,
                #[structopt(short, long, parse(from_occurrences))]
                pub quiet: i8,
                #[structopt(short, long, parse(from_occurrences))]
                pub verbose: i8,
            }
            fn _main_inner(#opt_name: #opt_type) -> ::qu::ick_use::Result {
                #body
            }
            fn main() {
                let opts: __wrapping_Opt = ::qu::ick_use::StructOpt::from_args();
                let log_level = match #default_log.saturating_add(opts.verbose).saturating_sub(opts.quiet) {
                    0 => ::qu::ick_use::log::LevelFilter::Off,
                    1 => ::qu::ick_use::log::LevelFilter::Error,
                    2 => ::qu::ick_use::log::LevelFilter::Warn,
                    3 => ::qu::ick_use::log::LevelFilter::Info,
                    4 => ::qu::ick_use::log::LevelFilter::Debug,
                    _ => ::qu::ick_use::log::LevelFilter::Trace,
                };
                ::qu::env_logger::builder()
                    .filter_level(log_level)
                    .init();
                if let Err(e) = _main_inner(opts.#opt_name) {
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
