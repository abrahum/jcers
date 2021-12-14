use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{self, Attribute, DataStruct, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(JceGet, attributes(jce))]
pub fn jce_get_derive(input: TokenStream) -> TokenStream {
    match parse_input(syn::parse_macro_input!(input as DeriveInput))
        .and_then(|(struct_name, struct_data)| gen_get_body(struct_name, struct_data.fields))
    {
        Ok(token) => token.into(),
        Err(errors) => to_compile_errors(errors).into(),
    }
}

#[proc_macro_derive(JcePut, attributes(jce))]
pub fn jce_put_derive(input: TokenStream) -> TokenStream {
    match parse_input(syn::parse_macro_input!(input as syn::DeriveInput))
        .and_then(|(struct_name, struct_data)| gen_put_body(struct_name, struct_data.fields))
    {
        Ok(token) => token.into(),
        Err(errors) => to_compile_errors(errors).into(),
    }
}

fn parse_input(input: DeriveInput) -> Result<(Ident, DataStruct), Vec<syn::Error>> {
    let struct_name = input.ident;
    let struct_data = input.data;
    if let syn::Data::Struct(data_struct) = struct_data {
        if let syn::Fields::Named(_) = data_struct.fields {
            Ok((struct_name, data_struct))
        } else {
            Err(vec![syn::Error::new(
                struct_name.span(),
                "only named fields are supported",
            )])
        }
    } else {
        Err(vec![syn::Error::new(
            struct_name.span(),
            "JceGet expected a struct",
        )])
    }
}

fn parse_attrs(attrs: &Vec<Attribute>, field: &Field) -> Result<u8, Vec<syn::Error>> {
    for meta in attrs.iter().map(|attr| attr.parse_meta()) {
        if let Ok(syn::Meta::List(list)) = meta {
            if list.path.is_ident("jce") {
                if let syn::NestedMeta::Lit(syn::Lit::Int(lit_int)) = list.nested.first().unwrap() {
                    return Ok(lit_int.base10_parse().unwrap());
                } else {
                    return Err(vec![syn::Error::new(
                        field.span(),
                        "jce attribute expected a number",
                    )]);
                }
            }
        }
    }
    Err(vec![syn::Error::new(
        field.span(),
        "JceGet expected a `jce` attribute",
    )])
}

fn gen_get_body(struct_name: Ident, fields: Fields) -> Result<TokenStream2, Vec<syn::Error>> {
    // let mut ts = TokenStream2::default();
    let mut idents_vec = vec![];
    let mut tys_vec = vec![];
    let mut tags_vec = vec![];
    for field in fields.iter() {
        tags_vec.push(parse_attrs(&field.attrs, &field)?);
        idents_vec.push(field.ident.as_ref().unwrap());
        tys_vec.push(&field.ty);
        // fields_vec.push((field.ident.as_ref().unwrap(), &field.ty, tag));
        // let ident = field.ident.as_ref().unwrap();
        // let ty = &field.ty;
    }
    Ok(quote! {
        impl jcers::JceGet for #struct_name {
            fn jce_get<B: bytes::Buf + ?Sized>(jce: &mut jcers::Jce<B>) -> jcers::JceResult<Self> {
                let sub = jce.head.ty == jcers::JceType::Struct;
                Ok(if sub {
                    let mut sub_jce = jce.sub_jce();
                    let r = #struct_name {
                        #(
                            #idents_vec: sub_jce.get_by_tag::<#tys_vec>(#tags_vec)?
                        ),*
                    };
                    jce.end_struct()?;
                    r
                } else {
                    #struct_name {
                        #(
                            #idents_vec: jce.get_by_tag::<#tys_vec>(#tags_vec)?
                        ),*
                    }
                })
            }

            fn empty() -> jcers::JceResult<Self> {
                Ok(#struct_name::default())
            }
        }
    })
}

fn gen_put_body(struct_name: Ident, fields: Fields) -> Result<TokenStream2, Vec<syn::Error>> {
    let mut ts = TokenStream2::default();
    for field in fields.iter() {
        let tag = parse_attrs(&field.attrs, &field)?;
        let ident = field.ident.as_ref().unwrap();
        ts = quote! {
            #ts
            self.#ident.jce_put(jce_mut, #tag);
        }
    }
    ts = quote! {
        impl jcers::JcePut for #struct_name {
            fn jce_put_raw(self, jce_mut: &mut jcers::JceMut) {
                #ts
            }
        }
    };
    Ok(ts)
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let errors = errors.into_iter().map(|e| e.to_compile_error());
    quote! {
        #(#errors)*
    }
}
