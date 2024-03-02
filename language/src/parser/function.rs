use crate::ast::{function::*, operand::Operand};
use syn::{
    parse::discouraged::Speculative,
    parse::{Parse, ParseStream, Result},
    Expr, Ident, Lit, Token,
};
impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        if let Ok(intrinsic) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Intrinsic(intrinsic));
        }
        let ident: Ident = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let inner = content.parse_terminated(Expr::parse, Token![,])?;
        Ok(Self::Ident(ident, inner.into_iter().collect()))
    }
}
impl Parse for Intrinsic {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::LocalAddress(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::ZeroExtend(el));
        }
        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SignExtend(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::ConditionalJump(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SetNFlag(el));
        }
        Ok(Self::SetZFlag(input.parse()?))
    }
}
impl Parse for FunctionCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Function = input.parse()?;

        let content;
        syn::parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, syn::token::Comma)?;
        let ret = Self {
            ident,
            args: args.into_iter().collect(),
        };

        Ok(ret)
    }
}


// =========================================================================
//                          Intrinsics parsing
// =========================================================================


impl Parse for LocalAddress {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "localaddress".to_owned() {
            return Err(input.error("localaddress"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let name: Lit = content.parse()?;
        let _: Token![,] = content.parse()?;
        let bits: Lit = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { name, bits })
    }
}
impl Parse for ZeroExtend {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "zeroextend".to_owned() {
            return Err(input.error("Expected zeroextend"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n: Ident = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand: op,
            bits: n,
        })
    }
}
impl Parse for SignExtend {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "signextend".to_owned() {
            return Err(input.error("Expected signextend"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n: Ident = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand: op,
            bits: n,
        })
    }
}
impl Parse for ConditionalJump {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "signextend".to_owned() {
            return Err(input.error("Expected signextend"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let condition: Ident = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand: op,
            condition,
        })
    }
}
impl Parse for SetNFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "setnflag".to_owned() {
            return Err(input.error("Expected setnflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { operand: op })
    }
}
impl Parse for SetZFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "setzflag".to_owned() {
            return Err(input.error("Expected setzflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { operand: op })
    }
}
