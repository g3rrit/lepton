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
      if let syn::Fields::Unnamed(fields) = &variant.fields {
        if let Some(ppair) = fields.unnamed.first() {
          let (field, _) = ppair.into_tuple();
          let ty = &field.ty; 
          result.push(quote! {
            #name::#id(_) => #ty::parse(),
          });
        }
      }
    }
    
    quote! {
      impl Parser for #name {
        fn parse(input: &mut NodeParser, env: &mut Env) -> Option<Rc<Node>> {
          match input.next() {
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
