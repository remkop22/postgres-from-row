use darling::ast::{self, Style};
use darling::{FromDeriveInput, FromField, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(FromRowTokioPostgres, attributes(from_row))]
pub fn derive_from_row_tokio_postgres(input: TokenStream) -> TokenStream {
    derive_from_row(input, quote::format_ident!("tokio_postgres"))
}

#[proc_macro_derive(FromRowPostgres, attributes(from_row))]
pub fn derive_from_row_postgres(input: TokenStream) -> TokenStream {
    derive_from_row(input, quote::format_ident!("postgres"))
}

fn derive_from_row(input: TokenStream, module: Ident) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    match try_derive_from_row(&derive_input, module) {
        Ok(result) => result,
        Err(err) => err.write_errors().into(),
    }
}

fn try_derive_from_row(input: &DeriveInput, module: Ident) -> Result<TokenStream, darling::Error> {
    let from_row_derive = DeriveFromRow::from_derive_input(input)?;
    from_row_derive.generate(module)
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(from_row),
    forward_attrs(allow, doc, cfg),
    supports(struct_named)
)]
struct DeriveFromRow {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), FromRowField>,
}

impl DeriveFromRow {
    fn generate(self, module: Ident) -> Result<TokenStream, darling::Error> {
        let ident = &self.ident;

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let fields = self
            .data
            .take_struct()
            .ok_or_else(|| darling::Error::unsupported_shape("enum").with_span(&self.ident))?;

        let fields = match fields.style {
            Style::Unit => {
                return Err(darling::Error::unsupported_shape("unit struct").with_span(&self.ident))
            }
            Style::Tuple => {
                return Err(darling::Error::unsupported_shape("tuple struct").with_span(&self.ident))
            }
            Style::Struct => fields.fields,
        };

        let from_row_fields = fields
            .iter()
            .map(|f| f.generate_from_row(&module))
            .collect::<syn::Result<Vec<_>>>()?;

        let try_from_row_fields = fields
            .iter()
            .map(|f| f.generate_try_from_row(&module))
            .collect::<syn::Result<Vec<_>>>()?;

        let original_predicates = where_clause.clone().map(|w| &w.predicates).into_iter();
        let mut predicates = Vec::new();

        for field in fields.iter() {
            let target_ty = &field.target_ty()?;
            let ty = &field.ty;
            predicates.push(if field.flatten {
                quote! (#target_ty: postgres_from_row::FromRow)
            } else {
                quote! (#target_ty: for<'a> #module::types::FromSql<'a>)
            });

            if field.from.is_some() {
                predicates.push(quote!(#ty: std::convert::From<#target_ty>))
            } else if field.try_from.is_some() {
                predicates.push(quote!(#ty: std::convert::From<#target_ty>))
            }
        }

        Ok(quote! {
            impl #impl_generics postgres_from_row::FromRow for #ident #ty_generics where #(#original_predicates),* #(#predicates),* {

                fn from_row(row: &#module::Row) -> Self {
                    Self {
                        #(#from_row_fields),*
                    }
                }

                fn try_from_row(row: &#module::Row) -> std::result::Result<Self, #module::Error> {
                    Ok(Self {
                        #(#try_from_row_fields),*
                    })
                }
            }
        }
        .into())
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(from_row), forward_attrs(allow, doc, cfg))]
struct FromRowField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    flatten: bool,
    try_from: Option<String>,
    from: Option<String>,
}

impl FromRowField {
    fn target_ty(&self) -> syn::Result<proc_macro2::TokenStream> {
        if let Some(from) = &self.from {
            Ok(from.parse()?)
        } else if let Some(try_from) = &self.try_from {
            Ok(try_from.parse()?)
        } else {
            Ok(self.ty.to_token_stream())
        }
    }

    fn generate_from_row(&self, module: &Ident) -> syn::Result<proc_macro2::TokenStream> {
        let ident = self.ident.as_ref().unwrap();
        let str_ident = ident.to_string();
        let field_ty = &self.ty;

        let target_ty = self.target_ty()?;

        let mut base = if self.flatten {
            quote!(<#target_ty as postgres_from_row::FromRow>::from_row(row))
        } else {
            quote!(#module::Row::get::<&str, #target_ty>(row, #str_ident))
        };

        if self.from.is_some() {
            base = quote!(<#field_ty as std::convert::From<#target_ty>>::from(#base));
        } else if self.try_from.is_some() {
            base = quote!(<#field_ty as std::convert::TryFrom<#target_ty>>::try_from(#base).expect("could not convert column"));
        };

        Ok(quote!(#ident: #base))
    }

    fn generate_try_from_row(&self, module: &Ident) -> syn::Result<proc_macro2::TokenStream> {
        let ident = self.ident.as_ref().unwrap();
        let str_ident = ident.to_string();
        let field_ty = &self.ty;
        let target_ty = self.target_ty()?;

        let mut base = if self.flatten {
            quote!(<#target_ty as postgres_from_row::FromRow>::try_from_row(row)?)
        } else {
            quote!(#module::Row::try_get::<&str, #target_ty>(row, #str_ident)?)
        };

        if self.from.is_some() {
            base = quote!(<#field_ty as std::convert::From<#target_ty>>::from(#base));
        } else if self.try_from.is_some() {
            base = quote!(<#field_ty as std::convert::TryFrom<#target_ty>>::try_from(#base)?);
        };

        Ok(quote!(#ident: #base))
    }
}
