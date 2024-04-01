use {
    proc_macro::TokenStream,
    proc_macro2::TokenStream as TokenStream2,
    quote::{quote, quote_spanned},
    syn::{
        punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Ident,
        LitInt, Meta, Token,
    },
};

#[proc_macro_derive(TransformDerive, attributes(wfrs_id))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let body: DeriveInput = syn::parse2(input.into()).unwrap();

    match body.data {
        Data::Struct(s) => serialize_struct(body.ident, s),
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
    .into()
}

fn serialize_struct(ident: Ident, s: DataStruct) -> TokenStream2 {
    let fields: Vec<(Ident, Vec<Attribute>)> = s
        .fields
        .iter()
        .map(|field| {
            (
                field.ident.clone().unwrap(),
                field.attrs.iter().map(|attr| attr.clone()).collect(),
            )
        })
        .collect();

    let mut match_branches = vec![];
    let mut vals = vec![];
    for field in fields {
        let (name, attrs) = field;

        // todo: expect for wfrs_id attribute
        // todo: check that field implements Transform and show error near field
        // todo: different behaivor for enums, plain types and slices
        // todo: check is it posible to generate serde attributes before serde to ignore fields

        for attr in attrs {
            if !attr.path().is_ident("wfrs_id") {
                continue;
            }

            match &attr.meta {
                Meta::List(list) if list.path.is_ident("wfrs_id") => {
                    match list.parse_args_with(Punctuated::<LitInt, Token![,]>::parse_terminated) {
                        Ok(val) => {
                            if val.len() != 1 {
                                return quote_spanned! {
                                    list.span() =>
                                    compile_error!("The `wfrs_id` must containt only one number");
                                };
                            }

                            let num: u8 = val[0].base10_parse().unwrap();
                            if vals.contains(&num) {
                                return quote_spanned! {
                                    list.span() => compile_error!("Duplicate `wfrs_id` value");
                                };
                            }
                            vals.push(num);
                            match_branches.push(quote! { #num => (&mut inside.#name as &mut dyn Transform).transform(value), });
                        }
                        Err(_) => {
                            return quote_spanned! {
                                list.span() =>
                                compile_error!("The `wfrs_id` attribute expects number literals to be comma separated");
                            };
                        }
                    }
                }
                meta => {
                    return quote_spanned! {
                        meta.span() =>
                        compile_error!("The `wfrs_id` attribute is the only supported argument");
                    };
                }
            }
        }
    }

    let res = quote! {
        impl Transform for Option<#ident> {
            fn transform(&mut self, params: &[Param]) {
                match self {
                    None => {
                        *self = Some(#ident {
                            ..Default::default()
                        });
                    }
                    Some(_) => (),
                }

                let params = match params.get(0).unwrap() {
                    Param::Child(child) => child,
                    _ => panic!("First param should be child param"),
                };

                if let Some(inside) = self {
                    for (key, value) in params.iter() {
                        match key {
                            #( #match_branches )*
                            v => (),
                            // v => panic!("Invalid wfrs id '{v}' for type {}", stringify!(#ident)),
                        }
                    }
                }
            }
        }
    };
    // println!("{}", res.to_string());
    res
}
