use crate::ast::{function::*, operand::Operand};
use quote::quote;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    parse_macro_input, Expr, Ident, Lit, LitStr, Token,
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
            return Ok(Self::Flag(el));
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
            return Ok(Self::SetNFlag(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SetCFlag(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SetVFlag(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Ror(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Sra(el));
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

impl Parse for Jump {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        println!("parsed Ident : {ident}");
        if ident.to_string().to_lowercase() != "jump".to_owned() {
            return Err(input.error("jump"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        println!("Parsing a jump function call");

        let target:Ident = content.parse()?;
        if input.peek(Token![,]) {
            let _: Token![,] = content.parse()?;
            let conditions = content.parse()?;
            if !content.is_empty() {
                return Err(content.error("Too many arguments"));
            }
            Ok(Self {
                target,
                condtion: Some(conditions),
            })
        } else {
            // if !content.is_empty() {
            //     return Err(content.error("Too many arguments"));
            // }
            Ok(Self {
                target,
                condtion: None,
            })
        }
    }
}

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

        let name: Lit = match content.peek(Ident) {
            // If name is an identifier, we conver the identifer to a string
            true => {
                let ident: Ident = content.parse()?;
                Lit::Str(LitStr::new(&ident.to_string(), ident.span()))
            }
            false => content.parse()?,
        };

        let _: Token![,] = content.parse()?;
        let bits: Lit = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { name, bits })
    }
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "flag".to_owned() {
            return Err(input.error("flag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);

        let name: Lit = match content.peek(Ident) {
            // If name is an identifier, we conver the identifer to a string
            true => {
                let ident: Ident = content.parse()?;
                Lit::Str(LitStr::new(&ident.to_string(), ident.span()))
            }
            false => content.parse()?,
        };
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { name })
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
        let n = content.parse()?;
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
        let n = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        let mut ret = Ok(Self {
            operand: op,
            bits: n,
        });
        ret
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
impl Parse for SetCFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "setcflag".to_owned() {
            return Err(input.error("Expected setcflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op1: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let op2: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let sub: Lit = content.parse()?;
        let _: Token![,] = content.parse()?;
        let carry: Lit = content.parse()?;

        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand1: op1,
            operand2: op2,
            sub,
            carry,
        })
    }
}
impl Parse for SetVFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "setvflag".to_owned() {
            return Err(input.error("Expected setvflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op1: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let op2: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let sub: Lit = content.parse()?;
        let _: Token![,] = content.parse()?;
        let carry: Lit = content.parse()?;

        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand1: op1,
            operand2: op2,
            sub,
            carry,
        })
    }
}

impl Parse for Ror {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "ror".to_owned() {
            return Err(input.error("ror"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        let ret = Ok(Self { operand: op, n });
        ret
    }
}

impl Parse for Sra {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase() != "sra".to_owned() {
            return Err(input.error("sra"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { operand: op, n })
    }
}
