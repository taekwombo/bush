#![feature(proc_macro_quote, proc_macro_tracked_env)]

use proc_macro::{quote, Literal, TokenTree, TokenStream};
use std::path::Path;

fn get_literal(ts: TokenStream) -> Result<Literal, TokenStream> {
    let mut iter = ts.into_iter();

    let Some(tt) = iter.next() else {
        return Err(quote! {
            compile_error!("Macro argument missing.")
        });
    };

    if iter.next().is_some() {
        return Err(quote! {
            compile_error!("Only one argument expected.")
        });
    }

    match tt {
        TokenTree::Literal(l) => Ok(l),
        TokenTree::Group(g) => get_literal(g.stream()),
        _ => {
            let err = TokenTree::Literal(Literal::string(&format!("Literal argument expected, got {tt:?}")));
            Err(quote! {
                compile_error!($err)
            })
        }
    }
}

#[proc_macro]
pub fn asset_path(tokens: TokenStream) -> TokenStream {
    proc_macro::tracked_env::var("ASSET_DIR").expect("Better to track ASSET_DIR env");

    let dir = Path::new(env!("ASSET_DIR"));
    let l = match get_literal(tokens) {
        Ok(l) => l,
        Err(e) => return e,
    };
    let chars = l.to_string();
    let mut chars = chars.chars();

    let path = match (chars.next(), chars.next_back()) {
        (Some('"'), Some('"')) => chars.as_str(),
        _ => {
            let err = TokenTree::Literal(Literal::string(&format!("Expected string literal, got {l}")));
            return quote! {
                compile_error!($err)
            }
        }
    };

    let path = dir.join(path);
    let Some(pstr) = path.as_path().to_str() else {
        return quote! {
            compile_error!("Invalid path provided {l}")
        }
    };

    if !path.is_file() {
        let err = TokenTree::Literal(Literal::string(&format!("Could not find file {pstr}")));
        return quote! {
            compile_error!($err)
        };
    }

    let path = TokenTree::Literal(Literal::string(pstr));
    quote! { $path }
}

