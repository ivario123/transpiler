pub mod function;
pub mod operand;
pub mod operation;

use crate::ast::*;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Result, Token};

use self::operations::BinOp;

impl Parse for IR {
    fn parse(input: ParseStream) -> Result<Self> {
        // Expected syntax : ret.extend[ .. ]
        let speculative = input.fork();
        let ret: Option<Ident> = match Ident::parse(&speculative) {
            Ok(ret) => {
                input.advance_to(&speculative);
                let _: Token![.] = input.parse()?;
                let token: Ident = input.parse()?;
                if token.to_string() != "extend".to_owned() {
                    return Err(input.error("Exptected extend"));
                }
                Some(ret)
            }
            _ => None,
        };
        let content;
        syn::bracketed!(content in input);
        let mut extensions: Vec<RustSyntax> = vec![];
        while !content.is_empty() {
            extensions.push(content.parse()?);
        }

        let ret = Self {
            ret,
            extensions: extensions.into_iter().collect(),
        };
        Ok(ret)
    }
}
impl Parse for RustSyntax {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![if]) {
            let _: Token![if] = input.parse()?;
            // Maaaasive limit, this should be expanded in the future
            let e: Ident = input.parse()?;
            let content;
            syn::braced!(content in input);
            let happy_case: Box<RustSyntax> = Box::new(content.parse()?);
            let sad_case = if input.peek(Token![else]) {
                let _: Token![else] = input.parse()?;
                let content;
                syn::braced!(content in input);
                Some(Box::new(content.parse()?))
            } else {
                None
            };
            return Ok(Self::If(e, happy_case, sad_case));
        }
        if input.peek(Token![for]) {
            let _: Token![for] = input.parse()?;
            let var: Ident = input.parse()?;
            let _: Token![in] = input.parse()?;
            let e: Expr = input.parse()?;
            let content;
            syn::braced!(content in input);
            let block: Box<RustSyntax> = Box::new(content.parse()?);
            return Ok(Self::For(var, e, block));
        }
        let mut ret: Vec<Box<IRExpr>> = vec![];
        while !input.is_empty() {
            if input.peek(Token![if]) | input.peek(Token![for]) {
                break;
            }
            let speculative = input.fork();
            match speculative.parse() {
                Ok(val) => {
                    input.advance_to(&speculative);
                    ret.push(Box::new(val));
                    let _: syn::token::Semi = input.parse()?;
                }
                Err(e) => {
                    if ret.len() != 0 {
                        break;
                    }
                    return Err(e);
                }
            }
        }
        Ok(Self::Exprs(ret))
    }
}

impl Parse for IRExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        if let Ok(unop) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::UnOp(unop));
        }

        let speculative = input.fork();
        if let Ok(assign) = speculative.parse() {
            let speculative_speculative = speculative.fork();
            let token = syn::token::Semi::parse(&speculative_speculative);
            match token {
                Ok(_) => {
                    input.advance_to(&speculative);
                    return Ok(Self::Assign(assign));
                }
                _ => {}
            }
        }

        let speculative = input.fork();
        if let Ok(func) = speculative.parse() {
            let speculative_speculative = speculative.fork();
            let token = syn::token::Semi::parse(&speculative_speculative);
            match token {
                Ok(_) => {
                    input.advance_to(&speculative);
                    return Ok(Self::Function(func));
                }
                _ => {}
            }
        }

        let binop: BinOp = input.parse()?;
        Ok(Self::BinOp(binop))
    }
}
