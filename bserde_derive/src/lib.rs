use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, parse_macro_input};

extern crate bserde as bserde;

/// # [`BSerialize`]
/// Macro for serializing
#[proc_macro_derive(BSerialize)]
pub fn bserialize(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let serialize_impl = if let Data::Struct(s) = &input.data {
    match &s.fields {
      Fields::Named(named) => {
        let fields = named.named.iter().map(|f| {
          let ident = f.ident.as_ref().unwrap();
          quote! {
              s.serialize_field(stringify!(#ident), &self.#ident)?;
          }
        });

        let fields_count = fields.len();

        quote! {
            impl #impl_generics bserde::BSerialize<'_> for #ident #ty_generics #where_clause {
                fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where
                    S: bserde::BSerializer {
                    use bserde::BSerializeStruct;
                    let mut s = serializer.serialize_struct(stringify!(#ident), #fields_count)?;
                    #(
                        #fields
                    )*
                    s.end()
                }
            }
        }
      }
      Fields::Unnamed(unamed) => {
        let fields = unamed.unnamed.iter().enumerate().map(|(i, _f)| {
          let index = Index::from(i);
          quote! {
              s.serialize_field(None, &self.#index)?;
          }
        });

        let fields_count = fields.len();

        quote! {
            impl #impl_generics bserde::BSerialize<'_> for #ident #ty_generics #where_clause {
                fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where
                    S: bserde::BSerializer {
                    use bserde::BSerializeStruct;
                    let mut s = serializer.serialize_struct(stringify!(#ident), #fields_count)?;
                    #(
                        #fields
                    )*
                    s.end()
                }
            }
        }
      }
      Fields::Unit => {
        quote! {
            compile_error!("Unit not supported !");
        }
      }
    }
  } else if let Data::Enum(_e) = &input.data {
    quote! {
        impl Copy for #ident {}
        impl Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl #impl_generics bserde::BSerialize<'_> for #ident #ty_generics #where_clause {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where
                S: bserde::BSerializer
            {
                use bserde::BSerializeEnum;
                let mut s = serializer.serialize_enum()?;
                s.serialize_variant(*self as u16)?;

                Ok(())
            }
        }
    }
  } else {
    unimplemented!()
  };

  quote! {
      #serialize_impl

      impl #impl_generics #ident #ty_generics #where_clause {
          fn save(&self, path: &str) -> std::io::Result<()> {
              use std::io::Write as _bserde_write;
              use bserde::BSerializer;
              use bserde::BSerialize;
              let fs = std::fs::File::create(path)?;
              let mut s = bserde::serializer::BinarySerializer::new();
              self.serialize(&mut s);
              s.save(fs)?;
              Ok(())
          }
      }
  }
  .into()
}

/// # BDeserialize
/// Deserializing macro
#[proc_macro_derive(BDeserialize)]
pub fn bdeserialize(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let deserialize_impl = if let Data::Struct(s) = &input.data {
    match &s.fields {
      Fields::Named(named) => {
        let fields = named.named.iter().map(|f| {
          let ident = f.ident.as_ref().unwrap();
          let ty = &f.ty;
          quote! {
              #ident: #ty::deserialize(input)?
          }
        });

        quote! {
            impl bserde::BDeserialize for #ident {
                type Error = std::io::Error;
                fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error> {
                    Ok(Self {
                        #(#fields,)*
                    })
                }
            }
        }
      }
      Fields::Unnamed(unamed) => {
        let fields = unamed.unnamed.iter().enumerate().map(|(i, f)| {
          let index = Index::from(i);
          let ty = &f.ty;
          quote! {
              #index: #ty::deserialize(input)?,
          }
        });

        quote! {
            impl bserde::BDeserialize for #ident {
                type Error = std::io::Error;
                fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error> {
                    Ok(Self {
                        #(#fields)*
                    })
                }
            }
        }
      }
      Fields::Unit => {
        quote! {
            compile_error!("Unit not supported !");
        }
      }
    }
  } else if let Data::Enum(e) = &input.data {
    let variants = e.variants.iter().enumerate().map(|(i,v)| {
        let vident = &v.ident;
        let i = i as u16;
        quote! {
            #i => #ident::#vident
        }        
    });

    quote! {
        impl bserde::BDeserialize for #ident {
            type Error = std::io::Error;
            fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error> {
                if input.len() < 2 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "not enough bytes for the enum discriminant"
                    ));
                }

                let bytes = &input[..2];
                let number = u16::from_le_bytes(bytes.try_into().unwrap());
                *input = &input[2..];
                let value = match number {
                    #(#variants,)*
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid enum discriminant"
                        ));
                    }
                };
                Ok(value)
            }
        }
    }
  } else {
    unimplemented!()
  };


  quote! {
      #deserialize_impl

      impl #impl_generics #ident #ty_generics #where_clause {
          fn read(path: &str) -> std::io::Result<Self> {
              use bserde::BDeserialize;
              use std::io::Read;
              let mut input_file = std::io::BufReader::new(std::fs::File::open(path)?);
              let mut buf = Vec::new();
              input_file.read_to_end(&mut buf)?;
              Self::deserialize(&mut buf.as_slice())
          }
      }
  }
  .into()
}
