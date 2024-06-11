use {
    proc_macro::TokenStream,
    proc_macro2::{TokenStream as TokenStream2, TokenTree},
    quote::{quote, quote_spanned},
    syn::{
        punctuated::Punctuated, spanned::Spanned, Data, DataStruct, DeriveInput, Expr, ExprArray,
        ExprLit, GenericArgument, Ident, Lit, Meta, MetaList, MetaNameValue, Path, PathArguments,
        Token, Type,
    },
};

#[derive(PartialEq)]
enum WfrsType {
    Transform,
    Preview,
}

struct WfrsData {
    typ: WfrsType,
    field: Option<Ident>,
    valid: bool,
    ids: Vec<u8>,
    names: Vec<String>,
    match_branches: Vec<TokenStream2>,
    method_calls: Vec<TokenStream2>,
    if_begin: Option<TokenStream2>,
    params: Vec<TokenStream2>,
}

impl Default for WfrsData {
    fn default() -> WfrsData {
        WfrsData {
            typ: WfrsType::Transform,
            field: None,
            valid: false,
            ids: vec![],
            names: vec![],
            match_branches: vec![],
            method_calls: vec![],
            if_begin: None,
            params: vec![],
        }
    }
}

#[proc_macro_derive(TransformDerive, attributes(wfrs))]
pub fn derive_transform(input: TokenStream) -> TokenStream {
    let body: DeriveInput = syn::parse2(input.into()).unwrap();

    match body.data {
        Data::Struct(s) => transform_struct(body.ident, s),
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
    .into()
}

fn transform_struct(ident: Ident, s: DataStruct) -> TokenStream2 {
    let mut data = WfrsData {
        typ: WfrsType::Transform,
        ..Default::default()
    };

    for field in s.fields {
        let name = field.ident.clone().unwrap();
        data.names.clear();
        data.field = Some(name.clone());

        // todo: check that field implements Transform and show error near field
        // todo: different behaivor for enums, plain types and slices
        // todo: check is it posible to generate serde attributes before serde to ignore fields
        // (seems like it's not)

        let mut has_wfrs = false;
        for attr in field.attrs {
            if !attr.path().is_ident("wfrs") {
                continue;
            }
            if has_wfrs {
                return quote_spanned! {
                    name.span() =>
                    compile_error!("Should have single `wfrs` argument");
                };
            }
            has_wfrs = true;

            match &attr.meta {
                Meta::List(list) if list.path.is_ident("wfrs") => {
                    if let Err(err) = data.wfrs(list) {
                        return err;
                    }
                }
                meta => {
                    return quote_spanned! {
                        meta.span() =>
                        compile_error!("The `wfrs` accepts only key-value list");
                    };
                }
            }
        }
        if !has_wfrs {
            return quote_spanned! {
                name.span() =>
                compile_error!("Should have `wfrs` argument");
            };
        }
    }

    let match_branches = data.match_branches;
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
    res
}

#[proc_macro_derive(PreviewDerive, attributes(wfrs))]
pub fn derive_preview(input: TokenStream) -> TokenStream {
    let body: DeriveInput = syn::parse2(input.into()).unwrap();

    match body.data {
        Data::Struct(s) => preview_struct(body.ident, s),
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
    .into()
}

fn preview_struct(ident: Ident, s: DataStruct) -> TokenStream2 {
    let mut data = WfrsData {
        typ: WfrsType::Preview,
        ..Default::default()
    };
    for field in s.fields {
        let name = field.ident.clone().unwrap();
        data.names.clear();
        data.params.clear();
        data.valid = false;

        match &field.ty {
            Type::Path(path) => {
                let segment = &path.path.segments[0];

                if &segment.ident == "Option" {
                    match &segment.arguments {
                        PathArguments::AngleBracketed(args) => match &args.args[0] {
                            GenericArgument::Type(Type::Path(path)) => {
                                let segment = &path.path.segments[0];

                                if !["ImgId", "NumberInRect", "bool"]
                                    .contains(&segment.ident.to_string().as_str())
                                {
                                    data.valid = true;
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        if !data.valid {
            continue;
        }
        let mut has_wfrs = false;
        for attr in field.attrs {
            if !attr.path().is_ident("wfrs") {
                continue;
            }
            if has_wfrs {
                return quote_spanned! {
                    name.span() =>
                    compile_error!("Should have single `wfrs` argument");
                };
            }
            has_wfrs = true;

            match &attr.meta {
                Meta::List(list) if list.path.is_ident("wfrs") => {
                    if let Err(err) = data.wfrs(list) {
                        return err;
                    }
                }
                meta => {
                    return quote_spanned! {
                        meta.span() =>
                        compile_error!("The `wfrs` accepts only key-value list");
                    };
                }
            }
        }
        let params = &data.params;
        let mut expr = quote! { res.append( &mut (&inside.#name as &dyn Preview).get_images(all_params, &vec![ #( #params )* ], images)); };
        if data.if_begin.is_some() {
            let if_begin = &data.if_begin;
            expr = quote! {
                #if_begin
                {
                    #expr
                }
            };
        }

        data.method_calls.push(expr);
    }
    let method_calls = data.method_calls;
    let res = quote! {
        impl Preview for Option<#ident> {
            fn get_images(&self,
                          all_params: &Option<PreviewParams>,
                          params: &Vec<ParamType>,
                          images: &Vec<Image>,) -> Vec<ImageWithCoords> {
                let mut res = vec![];

                if let Some(inside) = self {
                    if let Some(all_params_val) = &all_params {
                        #( #method_calls )*
                    }
                }
                res
            }
        }
    };
    if ["AnalogDialFace"].contains(&ident.to_string().as_str()) {
        println!("{}", &res);
    }
    res
}

impl WfrsData {
    fn wfrs_id(&mut self, value: &Expr) -> Result<(), TokenStream2> {
        match value {
            Expr::Lit(ExprLit {
                lit: Lit::Int(val), ..
            }) => {
                let id: u8 = val.base10_parse().unwrap();
                if self.ids.contains(&id) {
                    return Err(quote_spanned! {
                        val.span() => compile_error!("Duplicate `wfrs` value");
                    });
                }
                self.ids.push(id);
                let name = &self.field;
                self.match_branches.push(
                    quote! { #id => (&mut inside.#name as &mut dyn Transform).transform(value), },
                );
            }
            _ => {
                return Err(quote_spanned! {
                    value.span() => compile_error!("should be single number");
                })
            }
        }
        Ok(())
    }

    fn wfrs_params(&mut self, value: &Expr) -> Result<(), TokenStream2> {
        if self.valid {
            match value {
                Expr::Array(ExprArray { elems, .. }) => {
                    if elems.len() % 2 != 0 {
                        return Err(quote_spanned! {
                            value.span() => compile_error!("should have even amount of elements");
                        });
                    }
                    for i in 0..elems.len() / 2 {
                        let mut new_params = vec![];
                        let mut has_params = false;
                        let typ: Ident;
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(expr),
                            ..
                        }) = &elems[i * 2]
                        {
                            typ = expr.parse().unwrap();
                        } else {
                            return Err(quote_spanned! {
                                elems[i * 2].span() => compile_error!("Expected string");
                            });
                        }
                        let expr = &elems[i * 2 + 1];
                        let comma = if i > 0 {
                            quote! {,}
                        } else {
                            quote! {}
                        };
                        let mut res = quote! { #comma ParamType::#typ };
                        match expr {
                            Expr::Lit(ExprLit {
                                lit: Lit::Str(val), ..
                            }) => {
                                let mut single_literal = false;
                                let expr_val: TokenStream2 = val.parse().unwrap();
                                for token in expr_val {
                                    match &token {
                                        TokenTree::Ident(ident) => {
                                            single_literal = false;
                                            if ident == "param" {
                                                has_params = true;
                                                new_params.push(quote! { #token });
                                            } else {
                                                new_params.push(quote! { all_params_val.#ident });
                                            }
                                        }
                                        TokenTree::Literal(literal) => {
                                            single_literal = true;
                                            new_params.push(quote! { #token });
                                        }
                                        _ => {
                                            new_params.push(quote! { #token });
                                        }
                                    }
                                }
                                if has_params || single_literal {
                                    res.extend(quote! {( Some(#( #new_params )* ))});
                                } else {
                                    res.extend(quote! {( #( #new_params )* )});
                                }
                            }
                            _ => {
                                return Err(quote_spanned! {
                                    expr.span() => compile_error!("should be a string");
                                })
                            }
                        }
                        self.params.push(res);
                        if self.if_begin.is_none() && has_params {
                            self.if_begin = Some(quote! {
                            if let Some(ParamType::#typ(Some(param))) = params.get(0) })
                        }
                    }
                }
                _ => {
                    return Err(quote_spanned! {
                        value.span() => compile_error!("should be an array");
                    })
                }
            }
        }

        Ok(())
    }

    fn process_arg(&mut self, path: Path, value: Expr) -> Result<(), TokenStream2> {
        match &path {
            path if path.is_ident("id") => {
                if self.typ == WfrsType::Transform {
                    self.wfrs_id(&value)?;
                }
            }
            path if path.is_ident("params") => {
                if self.typ == WfrsType::Preview {
                    self.wfrs_params(&value)?;
                }
            }
            _ => {
                return Err(quote_spanned! {
                    path.span() => compile_error!("Not valid `wfrs` argument");
                })
            }
        }

        let name = path.get_ident().unwrap().to_string();
        if self.names.contains(&name) {
            return Err(quote_spanned! {
                path.span() => compile_error!("Duplicate `wfrs` arg");
            });
        }
        self.names.push(name);

        Ok(())
    }

    fn wfrs(&mut self, list: &MetaList) -> Result<(), TokenStream2> {
        let parsed = list.parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated);
        match parsed {
            Ok(args) => {
                for args_part in args {
                    let MetaNameValue { path, value, .. } = args_part;
                    self.process_arg(path, value)?;
                }
            }
            _ => {
                return Err(quote_spanned! {
                    list.span() =>
                    compile_error!("The `wfrs` attribute expects list of key-value pairs");
                });
            }
        }
        Ok(())
    }
}
