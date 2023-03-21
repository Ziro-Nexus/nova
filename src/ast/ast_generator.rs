use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote_spanned;
use quote::ToTokens;

use syn::Expr;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::token;

// getting custom magic keys
use super::magic;


// I want to create an Expr parser for example:
// let var_name = "hello world";
pub struct AllocatorExpr {
    let_sym : token::Let,
    var_name: syn::Ident,
    equal_sym: token::Eq,
    value: Expr,
    semicolon: token::Semi,
    span: Span,
}

impl Parse for AllocatorExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start_span = input.cursor().span();
        let let_sym = input.parse()?;
        let var_name = input.parse()?;
        let equal_sym = input.parse()?;
        let value: Expr = input.parse()?;
        let semicolon = input.parse()?;
        let end_span = input.cursor().span();
        let span = start_span.join(end_span).unwrap_or(start_span);
        

        let v: String = value.clone().into_token_stream().to_string();

        // I just need to geet the literal type!!!!
        match &value {
            
            _ => ()
        }
        
        Ok(Self {
            let_sym,
            var_name,
            equal_sym,
            value,
            semicolon,
            span,

        })
    }
}

impl ToTokens for AllocatorExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        
        let let_sym = &self.let_sym;
        let var_name = &self.var_name;
        let equal_sym = &self.equal_sym;
        let value = &self.value;
        let semicolon = &self.semicolon;
        let span = self.span;
 

        tokens.extend(quote_spanned! { span =>
           #let_sym
           #var_name
           #equal_sym
           #value
           #semicolon

        });
    }
}

impl AllocatorExpr {
    pub fn translate(input: &str) -> Option<TokenStream > {
        let tokens = syn::parse_str::<AllocatorExpr>(input);
        if let Err(e) = tokens {
            println!("An erros has occured for Allocation Expr: \n{}", e.to_string());
            return None;
        }

        let tokens = tokens.unwrap();
        Some(tokens.into_token_stream())
    }
}

struct AstGen {
    allocator_expr: AllocatorExpr
}