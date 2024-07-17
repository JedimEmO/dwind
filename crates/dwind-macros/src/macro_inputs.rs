use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Ident, LitStr, Token, Expr};

pub struct DwindInput {
    pub self_ident: Ident,
    pub classes: LitStr,
}

impl Parse for DwindInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let self_ident = input.parse::<Ident>()?;
        let _sep = input.parse::<Token![,]>()?;
        let classes = input.parse::<LitStr>()?;

        Ok(DwindInput {
            self_ident,
            classes,
        })
    }
}


pub struct DwindInputSignal {
    pub input: DwindInput,
    pub signal: syn::Expr
}

impl Parse for crate::macro_inputs::DwindInputSignal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_value = input.parse::<DwindInput>()?;
        let _sep = input.parse::<Token![,]>()?;
        let signal = input.parse::<Expr>()?;

        Ok(Self {
            input: input_value,
            signal
        })
    }
}

pub struct DwGenerateInput {
    pub class_name: LitStr,
    pub class_definition: LitStr,
}

impl Parse for DwGenerateInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let class_name = input.parse::<LitStr>()?;
        let _sep = input.parse::<Token![,]>()?;
        let classes = input.parse::<LitStr>()?;

        Ok(DwGenerateInput {
            class_name,
            class_definition: classes,
        })
    }
}

#[derive(Debug)]
pub struct StrLitPair {
    pub first: syn::LitStr,
    pub second: syn::LitStr,
}

impl Parse for StrLitPair {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        let first = content.parse::<syn::LitStr>()?;
        let _sep = content.parse::<Token![,]>()?;
        let second = content.parse::<syn::LitStr>()?;

        Ok(Self { first, second })
    }
}

#[derive(Debug)]
pub struct ArrayOfTuples {
    pub tuples: Punctuated<StrLitPair, Token![,]>,
}

impl Parse for ArrayOfTuples {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);

        let content = content.parse_terminated(StrLitPair::parse, Token![,])?;

        Ok(Self { tuples: content })
    }
}

#[derive(Debug)]
pub struct DwGenerateMapInput {
    pub base_output_ident: syn::LitStr,
    pub dwind_class_lit: syn::LitStr,
    pub args: ArrayOfTuples,
}

impl Parse for DwGenerateMapInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let base_output_ident = input.parse::<syn::LitStr>()?;
        let _sep = input.parse::<Token![,]>()?;
        let dwind_class_lit = input.parse::<syn::LitStr>()?;
        let _sep = input.parse::<Token![,]>()?;
        let args = input.parse::<ArrayOfTuples>()?;

        Ok(Self {
            base_output_ident,
            dwind_class_lit,
            args,
        })
    }
}
