extern crate proc_macro;
extern crate quote;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumParser)]
pub fn enum_parse(input: TokenStream) -> TokenStream {
  let ast = syn::parse_macro_input!(input as syn::ItemEnum);
  

  let gen = {
    let name = &ast.ident;
    let variants = &ast.variants;
    let mut result = Vec::new();
    
    for (_, variant) in variants.iter().enumerate() {
      let id = &variant.ident;
      result.push(quote! {
        #name::#id(val) => val.parse(),
      });
    }
    
    quote! {
      impl Parser for #name {
        fn parse(&self) -> Option<()> {
          match self {
            #(#result)*
          }
        }
      }
    }
  };

  gen.into()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
