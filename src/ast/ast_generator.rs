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



/*
                ALLOCATOR GRAMMAR - STRUCT TO PARSE VARIABLE DECLARATIONS TO AST TREE
*/
// I want to create an Expr parser for example:
// let var_name = "hello world";
pub struct AllocatorGrammar {
    let_sym : magic::local,
    var_name: syn::Ident,
    equal_sym: token::Eq,
    value: Expr,
    semicolon: token::Semi,
    span: Span,
}

impl Parse for AllocatorGrammar {
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

impl ToTokens for AllocatorGrammar {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        
        let let_sym = &self.let_sym;
        let var_name = &self.var_name;
        let equal_sym = &self.equal_sym;
        let value = &self.value;
        let semicolon = &self.semicolon;
        let span = self.span;
        let num = 1;
 

        tokens.extend(quote_spanned! { span =>
           #let_sym
           #var_name
           #equal_sym
           #value
           #semicolon
           #num
        });
    }
}

impl AllocatorGrammar {
    pub fn translate(input: &str) -> Result<TokenStream, String> {
        let tokens = syn::parse_str::<AllocatorGrammar>(input);
        if let Err(e) = tokens {
            return Err(e.to_string().clone());
        }

        let tokens = tokens.unwrap();
        Ok(tokens.into_token_stream())
    }
}

/*
                                ALLOCATOR GRAMMAR - END BLOCK
*/