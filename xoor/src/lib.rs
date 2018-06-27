#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::tokenstream::TokenTree;
use syntax::parse::token::{self, Lit};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::quote::rt::Span;
use syntax::ext::build::AstBuilder;

fn expand_xoor(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    if args.len() != 1 {
        cx.span_err(sp, "there should only be one argument");
        
        return DummyResult::any(sp);
    }

    let encrypted = match args[0] {
        TokenTree::Token(_, token::Literal(Lit::Str_(input), _)) => {
            let pre_rev = input.as_str();

            pre_rev
                .bytes()
                .map(|x| x ^ 127)
                .map(|x| cx.expr_u8(sp, x))
                .collect()
        }
        _ => {
            cx.span_err(sp, "argument should be a string literal");

            return DummyResult::any(sp);
        }
    };

    MacEager::expr(cx.expr_vec_slice(sp, encrypted))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("xoor", expand_xoor);
}
