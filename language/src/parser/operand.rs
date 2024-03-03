use crate::ast::operand::*;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    token::Let,
    Expr, Ident, Token,
};
impl Parse for ExprOperand {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let inner: Expr = content.parse()?;
            // This needs to be cleaned up
            if input.peek(Token![.]) {
                let mut ops = vec![];
                while input.peek(Token![.]) {
                    let _: Token![.] = input.parse()?;
                    let fident: Ident = input.parse()?;
                    if input.peek(syn::token::Paren) {
                        let content;
                        syn::parenthesized!(content in input);
                        let operands =
                            content.parse_terminated(Operand::parse, syn::token::Semi)?;
                        ops.push((
                            fident,
                            operands.into_iter().map(|el| Box::new(el)).collect(),
                        ));
                        continue;
                    }
                    return Err(input.error("Expected function arguments"));
                }
                // Chain(Box<ExprOperand>, Vec<(Ident, Vec<Box<Operand>>)>),
                return Ok(Self::Chain(Box::new(Self::Paren(inner)), ops));
            }
            return Ok(Self::Paren(inner));
        }
        let speculative = input.fork();

        if let Ok(literal) = speculative.parse() {
            input.advance_to(&speculative);
            // This needs to be cleaned up
            if input.peek(Token![.]) {
                let mut ops = vec![];
                while input.peek(Token![.]) {
                    let _: Token![.] = input.parse()?;
                    let fident: Ident = input.parse()?;
                    if input.peek(syn::token::Paren) {
                        let content;
                        syn::parenthesized!(content in input);
                        let operands =
                            content.parse_terminated(Operand::parse, syn::token::Semi)?;
                        ops.push((
                            fident,
                            operands.into_iter().map(|el| Box::new(el)).collect(),
                        ));
                        continue;
                    }
                    return Err(input.error("Expected function arguments"));
                }
                // Chain(Box<ExprOperand>, Vec<(Ident, Vec<Box<Operand>>)>),
                return Ok(Self::Chain(Box::new(Self::Literal(literal)), ops));
            }
            return Ok(Self::Literal(literal));
        }
        let speculative = input.fork();
        let ident = speculative.parse()?;
        if speculative.peek(Token![.]) {
            input.advance_to(&speculative);
            let mut ops = vec![];
            while input.peek(Token![.]) {
                let _: Token![.] = input.parse()?;
                let fident: Ident = input.parse()?;
                if input.peek(syn::token::Paren) {
                    let content;
                    syn::parenthesized!(content in input);
                    let operands = content.parse_terminated(Operand::parse, syn::token::Semi)?;
                    ops.push((
                        fident,
                        operands.into_iter().map(|el| Box::new(el)).collect(),
                    ));
                    continue;
                }
                return Err(input.error("Expected function arguments"));
            }
            // Chain(Box<ExprOperand>, Vec<(Ident, Vec<Box<Operand>>)>),
            return Ok(Self::Chain(Box::new(Self::Ident(ident)), ops));
        }
        let speculative_f = input.fork();
        if let Ok(function_call) = speculative_f.parse() {
            input.advance_to(&speculative_f);
            if input.peek(Token![.]) {
                let mut ops = vec![];
                while input.peek(Token![.]) {
                    let _: Token![.] = input.parse()?;
                    let fident: Ident = input.parse()?;
                    if input.peek(syn::token::Paren) {
                        let content;
                        syn::parenthesized!(content in input);
                        let operands =
                            content.parse_terminated(Operand::parse, syn::token::Semi)?;
                        ops.push((
                            fident,
                            operands.into_iter().map(|el| Box::new(el)).collect(),
                        ));
                        continue;
                    }
                    return Err(input.error("Expected function arguments"));
                }
                // Chain(Box<ExprOperand>, Vec<(Ident, Vec<Box<Operand>>)>),
                return Ok(Self::Chain(
                    Box::new(Self::FunctionCall(function_call)),
                    ops,
                ));
            }
            return Ok(Self::FunctionCall(function_call));
        }
        input.advance_to(&speculative);
        Ok(Self::Ident(ident))
    }
}

impl Parse for Operand {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        if let Ok(val) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::FunctionCall(val));
        }
        let speculative = input.fork();
        if let Ok(val) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Expr(val));
        }
        let speculative = input.fork();
        if let Ok(val) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Ident(val));
        }

        // let speculative = input.fork();
        // if let Ok(val) = speculative.parse() {
        //     input.advance_to(&speculative);
        //     return Ok(Self::IRExpr(val));
        // }
        Err(input.error("Expected operand"))
    }
}

impl Parse for IdentOperand {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Let) {
            let _: Let = input.parse()?;
            let ident: Ident = input.parse()?;
            return Ok(Self {
                define: true,
                ident,
            });
        }
        let ident: Ident = input.parse()?;
        Ok(Self {
            define: false,
            ident,
        })
    }
}
