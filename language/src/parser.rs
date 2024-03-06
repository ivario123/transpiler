pub mod function;
pub mod operand;
pub mod operation;

use crate::ast::function::Jump;
use crate::ast::*;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Expr, Ident, Lit, Result, Token};

use self::operations::BinOp;

impl IR {
    fn parse_internal(input: ParseStream) -> Result<Self> {
        // Expected syntax : ret.extend[ .. ]
        let speculative = input.fork();
        let ret: Option<Ident> = match Ident::parse(&speculative) {
            Ok(ret) => match syn::token::Dot::parse(&speculative) {
                Ok(_) => {
                    input.advance_to(&speculative);

                    let token: Ident = input.parse()?;
                    if token.to_string() != "extend".to_owned() {
                        return Err(input.error("Exptected extend"));
                    }
                    Some(ret)
                }
                _ => None,
            },
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
impl Parse for IR {
    fn parse(input: ParseStream) -> Result<Self> {
        let ret = match Self::parse_internal(input) {
            Ok(val) => val,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(ret)
    }
}
impl Parse for RustSyntax {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![if]) {
            let _: Token![if] = input.parse()?;
            // Simply require parenthesise here, this is a bit of a "fulhack"
            // but it works for now
            let content;
            parenthesized!(content in input);

            let e: Expr = content.parse()?;
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
                    let _: syn::token::Semi = match speculative.parse(){
                        Ok(t) => t,
                        Err(_) => return Err(speculative.error("Expected `;`"))

                    };
                    input.advance_to(&speculative);
                    ret.push(Box::new(val));
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
        println!("Parsing an IRExpr from {input}");
        let speculative = input.fork();
        if let Ok(unop) = speculative.parse() {
            println!("  Found unop {unop:?}");
            println!("  Remaning tokens : {speculative}");
            input.advance_to(&speculative);
            return Ok(Self::UnOp(unop));
        }

        let speculative = input.fork();
        if let Ok(assign) = speculative.parse() {
            println!("  Found assign {assign:?}");
            println!("  Remaning tokens : {speculative}");
            input.advance_to(&speculative);
            return Ok(Self::Assign(assign));
        }

        let speculative = input.fork();
        if let Ok(res) = speculative.parse() {
            println!("  Found binop {res:?}");
            println!("  Remaning tokens : {speculative}");
            input.advance_to(&speculative);
            return Ok(Self::BinOp(res));
        }

        let speculative = input.fork();
        if let Ok(res) = speculative.parse() {
            println!("  Found jump {res:?}");
            println!("  Remaning tokens : {speculative}");
            input.advance_to(&speculative);
            return Ok(Self::Jump(res));
        }

        let speculative = input.fork();
        if let Ok(func) = speculative.parse() {
            println!("  Found function call {func:?}");
            println!("  Remaning tokens : {speculative}");
            input.advance_to(&speculative);
            return Ok(Self::Function(func));
        }
        Err(input.error("Expected a valid IRExpr here"))
    }
}
