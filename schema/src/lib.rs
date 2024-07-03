//! # `npsd` Derive Macros
//! 
//! This module provides a set of custom derive macros to simplify the implementation of traits required by the `npsd` framework.
//! These macros automate the generation of boilerplate code for various payload processing tasks, including serialization, deserialization, and payload conversion.
//!
//! ## Available Macros
//!
//! ### `#[derive(Info)]`
//! Generates an implementation of the `PayloadInfo` trait, which provides metadata about the payload type.
//!
//! ### `#[derive(Schema)]`
//! Generates implementations for payload processing traits such as `IntoPayload`, `FromPayload`, and `Payload` for public use.
//!
//! ### `#[derive(Bitmap)]`
//! Generates implementations for payload processing traits for bitmap structures with up to 8 fields.
//!
//! ### `#[derive(AsyncSchema)]`
//! Generates asynchronous implementations for payload processing traits such as `AsyncIntoPayload`, `AsyncFromPayload`, and `AsyncPayload` for public use.
//!
//! ### `#[derive(AsyncBitmap)]`
//! Generates asynchronous implementations for payload processing traits for bitmap structures with up to 8 fields.

#[doc(hidden)]
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, token::Plus, Data, DataEnum, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Generics, Ident, Index, Lifetime, LifetimeParam, TypeParamBound};
#[doc(hidden)]
use quote::{quote, quote_spanned};
#[doc(hidden)]
use proc_macro::TokenStream;
#[doc(hidden)]
use proc_macro2::Span;

const DEFAULT_LIFETIME: &'static str = "'__payload";
const DEFAULT_SCOPE_LIFETIME: &'static str = "'__payload_scope";
const DEFAULT_CONTEXT: &'static str = "__PayloadCtx";
const DEFAULT_MIDDLEWARE: &'static str = "__PayloadMw";

#[doc(hidden)]
fn resolve_lifetime(generics: &Generics, lifetime_name: &str) -> (bool, Lifetime) {
    if let Some(existing_lifetime) = generics.params.iter().find_map(|param| {
        if let GenericParam::Lifetime(lifetime_param) = param {
            Some(lifetime_param.lifetime.clone())
        } else {
            None
        }
    }) {
        return (true, existing_lifetime);
    }

    (false, Lifetime::new(lifetime_name, Span::call_site()))
}

#[doc(hidden)]
fn has_bound(bounds: &Punctuated<TypeParamBound, Plus>, bound_to_check: &str) -> bool {
    bounds.iter().any(|bound| {
        if let TypeParamBound::Trait(trait_bound) = bound {
            trait_bound.path.segments.iter().any(|segment| segment.ident == bound_to_check)
        } else {
            false
        }
    })
}

#[doc(hidden)]
fn schema_into_impl(generics: &mut Generics, internal: bool, context: &Ident) {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            if type_param.ident == DEFAULT_CONTEXT {
                continue;
            }

            if !has_bound(&type_param.bounds, "IntoPayload") {
                type_param.bounds.push(if internal {
                    parse_quote!(IntoPayload<#context>)
                } else {
                    parse_quote!(npsd::IntoPayload<#context>)
                });
            }
        }
    }
}

#[doc(hidden)]
fn schema_from_impl(generics: &mut Generics, internal: bool, lifetime: &Lifetime, context: &Ident) {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            if type_param.ident == DEFAULT_CONTEXT {
                continue;
            }

            if !has_bound(&type_param.bounds, "FromPayload") {
                type_param.bounds.push(if internal {
                    parse_quote!(FromPayload<#lifetime, #context>)
                } else {
                    parse_quote!(npsd::FromPayload<#lifetime, #context>)
                });
            }
        }
    }
}

#[doc(hidden)]
fn schema_payload_impl(generics: &mut Generics, internal: bool, context: &Ident) {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            if type_param.ident == DEFAULT_CONTEXT {
                continue;
            }

            if !has_bound(&type_param.bounds, "Payload") {
                type_param.bounds.push(if internal {
                    parse_quote!(Payload<#context>)
                } else {
                    parse_quote!(npsd::Payload<#context>)
                });
            }
        }
    }
}

#[doc(hidden)]
fn async_schema_into_impl(generics: &mut Generics, internal: bool, context: &Ident) {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            if type_param.ident == DEFAULT_CONTEXT {
                continue;
            }

            if !has_bound(&type_param.bounds, "AsyncIntoPayload") {
                type_param.bounds.push(if internal {
                    parse_quote!(AsyncIntoPayload<#context>)
                } else {
                    parse_quote!(npsd::AsyncIntoPayload<#context>)
                });
            }
        }
    }
}

#[doc(hidden)]
fn async_schema_from_impl(generics: &mut Generics, internal: bool, lifetime: &Lifetime, context: &Ident) {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            if type_param.ident == DEFAULT_CONTEXT {
                continue;
            }

            if !has_bound(&type_param.bounds, "AsyncFromPayload") {
                type_param.bounds.push(if internal {
                    parse_quote!(AsyncFromPayload<#lifetime, #context>)
                } else {
                    parse_quote!(npsd::AsyncFromPayload<#lifetime, #context>)
                });
            }
        }
    }
}

#[doc(hidden)]
fn async_schema_payload_impl(generics: &mut Generics, internal: bool, context: &Ident) {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            if type_param.ident == DEFAULT_CONTEXT {
                continue;
            }

            if !has_bound(&type_param.bounds, "AsyncPayload") {
                type_param.bounds.push(if internal {
                    parse_quote!(AsyncPayload<#context>)
                } else {
                    parse_quote!(npsd::AsyncPayload<#context>)
                });
            }
        }
    }
}

#[doc(hidden)]
#[proc_macro_derive(Info)]
pub fn payload_info_public_impl(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, generics, .. } = parse_macro_input!(input);
    let (generics_impl, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #generics_impl npsd::PayloadInfo for #ident #ty_generics #where_clause {
            const TYPE: &'static str = stringify!(#ident);
        }
    };

    gen.into()
}

#[doc(hidden)]
#[proc_macro_derive(InfoInternal)]
pub fn payload_info_intenal_impl(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, generics, .. } = parse_macro_input!(input);
    let (generics_impl, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #generics_impl PayloadInfo for #ident #ty_generics #where_clause {
            const TYPE: &'static str = stringify!(#ident);
        }
    };

    gen.into()
}

#[proc_macro_derive(Schema)]
pub fn schema_public_impl(input: TokenStream) -> TokenStream {
    schema_impl(input, false)
}

#[doc(hidden)]
#[proc_macro_derive(SchemaInternal)]
pub fn schema_internal_impl(input: TokenStream) -> TokenStream {
    schema_impl(input, true)
}

#[doc(hidden)]
fn schema_impl(input: TokenStream, internal: bool) -> TokenStream {
    let DeriveInput { ident, data, generics, .. } = parse_macro_input!(input);
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let (lifetime_exist, lifetime) = resolve_lifetime(&generics, DEFAULT_LIFETIME);
    let context = Ident::new(DEFAULT_CONTEXT, Span::call_site());
    let scope = Lifetime::new(DEFAULT_SCOPE_LIFETIME, Span::call_site());
    let mw = Ident::new(DEFAULT_MIDDLEWARE, Span::call_site());
    let mut context_generics = generics.clone();

    let context_param: GenericParam = syn::parse_quote!(#context);
    context_generics.params.push(context_param);

    let mut into_generics = context_generics.clone();
    let mut from_generics = context_generics.clone();
    let mut payload_generics = context_generics.clone();

    if !lifetime_exist {
        let lifetime_param = LifetimeParam::new(lifetime.clone());
        from_generics.params.insert(0, GenericParam::Lifetime(lifetime_param));
    }

    schema_into_impl(&mut into_generics, internal, &context);
    let (into_impl, _, _) = into_generics.split_for_impl();

    schema_from_impl(&mut from_generics, internal, &lifetime, &context);
    let (from_impl, _, _) = from_generics.split_for_impl();

    schema_payload_impl(&mut payload_generics, internal, &context);
    let (payload_impl, _, _) = payload_generics.split_for_impl();

    let sender_block = match data.clone() {
        Data::Struct(data_struct) => {
            let fields = match data_struct.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    named.iter().map(|f| {
                        let name = &f.ident;
                        let span = f.span();

                        quote_spanned! { span =>
                            next.into_payload(&self.#name, ctx)?;
                        }
                    }).collect::<Vec<_>>()
                },

                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    unnamed.iter().enumerate().map(|(i, _)| {
                        let index = Index::from(i);
                        let span = index.span();

                        quote_spanned! { span =>
                            next.into_payload(&self.#index, ctx)?;
                        }
                    }).collect::<Vec<_>>()
                },

                Fields::Unit => Vec::new(),
            };

            quote! { #( #fields )* }
        },
        Data::Enum(DataEnum { variants, .. }) => {
            let variant_cases = variants.iter().enumerate().map(|(index, variant)| {
                let variant_ident = &variant.ident;
                let variant_span = variant.span(); 

                match &variant.fields {
                    Fields::Named(FieldsNamed { named, .. }) => {
                        let (field_patterns, field_serializations): (Vec<_>, Vec<_>) = named.iter()
                            .map(|f| {
                                let name = f.ident.as_ref().unwrap();
                                let span = name.span();
                                let pattern = quote_spanned! { span => #name };
                                let serialization = quote_spanned! { span => next.into_payload(&#name, ctx)?; };
                                (pattern, serialization)
                            }).unzip();

                        quote_spanned! { variant_span => 
                            #ident::#variant_ident { #(#field_patterns,)* } => {
                                next.into_payload(&#index, ctx)?;
                                #( #field_serializations )*
                            }
                        }
                    },
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let (field_patterns, field_serializations): (Vec<_>, Vec<_>) = unnamed.iter().enumerate()
                            .map(|(i, _)| {
                                let field_name = Ident::new(&format!("__self_{}", i), Span::call_site());
                                let pattern = quote! { #field_name };
                                let serialization = quote! { next.into_payload(&#field_name, ctx)?; };
                                (pattern, serialization)
                            }).unzip();
                    
                        quote_spanned! { variant_span => 
                            #ident::#variant_ident( #( #field_patterns, )* ) => {
                                next.into_payload(&#index, ctx)?;
                                #( #field_serializations )*
                            }
                        }
                    },
                    Fields::Unit => {
                        quote_spanned! { variant_span => 
                            #ident::#variant_ident => {
                                next.into_payload(&#index, ctx)?;
                            }
                        }
                    },
                }
            });

            quote! {
                match self {
                    #( #variant_cases, )*
                }
            }
        },
        Data::Union(_) => {
            return quote! {
                compile_error!("Union types are not supported by this macro.");
            }.into();
        },
    };

    let receiver_block = match data.clone() {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    let fields = named.iter().map(|f| {
                        let field = &f.ident;
                        let ty = &f.ty;
                        let span = f.span();

                        quote_spanned! { span =>
                            #field: next.from_payload::<#context, #ty>(ctx)? // as #ty
                        }
                    }).collect::<Vec<_>>();

                    quote! {
                        Ok(#ident {
                            #( #fields ),*
                        })
                    }
                },
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let fields = unnamed.iter().enumerate().map(|(_, f)| {
                        let ty = &f.ty;

                        quote! {
                            next.from_payload::<#context, #ty>(ctx)? // as #ty
                        }
                    }).collect::<Vec<_>>();

                    quote! {
                        Ok(#ident (
                            #( #fields ),*
                        ))
                    }
                },
                Fields::Unit => {
                    quote! {
                        Ok(#ident)
                    }
                },
            }
        },
        Data::Enum(DataEnum { variants, .. }) => {
            let match_variants = variants.iter().enumerate().map(|(index, variant)| {
                let variant_ident = &variant.ident;
                
                match &variant.fields {
                    Fields::Named(FieldsNamed { named, .. }) => {
                        let deserializations = named.iter().map(|f| {
                            let name = &f.ident;
                            let ty = &f.ty;
        
                            quote! {
                                #name: next.from_payload::<#context, #ty>(ctx)? // as #ty
                            }
                        });
                        
                        quote! {
                            #index => Ok(#ident::#variant_ident { #(#deserializations),* })
                        }
                    },
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let deserializations = unnamed.iter().map(|f| {
                            let ty = &f.ty;
        
                            quote! {
                                next.from_payload::<#context, #ty>(ctx)? // as #ty
                            }
                        });
                        
                        quote! {
                            #index => Ok(#ident::#variant_ident( #(#deserializations),* ))
                        }
                    },
                    Fields::Unit => {
                        quote! {
                            #index => Ok(#ident::#variant_ident)
                        }
                    },
                }
            }).collect::<Vec<_>>();
        
            if internal {
                quote! {
                    let variant_index: usize = next.from_payload(ctx)?;
            
                    match variant_index {
                        #(#match_variants,)*
                        _ => Err(Error::UnknownVariant("Index out of bounds for enum".to_string())),
                    }
                }
            } else {
                quote! {
                    let variant_index: usize = next.from_payload(ctx)?;
            
                    match variant_index {
                        #(#match_variants,)*
                        _ => Err(npsd::Error::UnknownVariant("Index out of bounds for enum".to_string())),
                    }
                }
            }
        },
        Data::Union(_) => {
            return quote! {
                compile_error!("Union types are not supported by this macro.");
            }.into();
        },
    };

    let gen = if internal {
        quote! {
            impl #into_impl IntoPayload<#context> for #ident #ty_generics #where_clause {
                fn into_payload<#mw: Middleware>(&self, ctx: &mut #context, next: &mut #mw) -> Result<(), Error> {
                    #sender_block
                    Ok(())
                }
            }

            impl #from_impl FromPayload<#lifetime, #context> for #ident #ty_generics #where_clause {
                fn from_payload<#scope, #mw: Middleware>(ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, Error>
                    where
                        #lifetime: #scope
                {
                    #receiver_block
                }
            }

            impl #payload_impl Payload<#context> for #ident #ty_generics #where_clause {}
        }
    } else {
        quote! {
            impl #into_impl npsd::IntoPayload<#context> for #ident #ty_generics #where_clause {
                fn into_payload<#mw: npsd::Middleware>(&self, ctx: &mut #context, next: &mut #mw) -> Result<(), npsd::Error> {
                    #sender_block
                    Ok(())
                }
            }

            impl #from_impl npsd::FromPayload<#lifetime, #context> for #ident #ty_generics #where_clause {
                fn from_payload<#scope, #mw: npsd::Middleware>(ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, npsd::Error>
                    where
                        #lifetime: #scope
                {
                    #receiver_block
                }
            }

            impl #payload_impl npsd::Payload<#context> for #ident #ty_generics #where_clause {}
        }
    };

    gen.into()
}

#[proc_macro_derive(Bitmap)]
pub fn bitmap_derive(input: TokenStream) -> TokenStream {
    bitmap_impl(input, false)
}

#[doc(hidden)]
#[proc_macro_derive(BitmapInternal)]
pub fn bitmap_internal_derive(input: TokenStream) -> TokenStream {
    bitmap_impl(input, true)
}

#[doc(hidden)]
fn bitmap_impl(input: TokenStream, internal: bool) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        Data::Struct(ref data_struct) => &data_struct.fields,
        _ => {
            return quote! {
                compile_error!("Bitmap can only be derived for structs with named or unnamed fields");
            }.into();
        } 
    };

    let field_count = match fields {
        Fields::Named(ref named_fields) => named_fields.named.len(),
        Fields::Unnamed(ref unnamed_fields) => unnamed_fields.unnamed.len(),
        Fields::Unit => 0,
    };

    if field_count > 8 {
        return quote! {
            compile_error!("Bitmap can only be derived for structs with no more than 8 fields");
        }.into();
    }

    let lifetime = Lifetime::new(DEFAULT_LIFETIME, Span::call_site());
    let scope = Lifetime::new(DEFAULT_SCOPE_LIFETIME, Span::call_site());

    let context = Ident::new(DEFAULT_CONTEXT, Span::call_site());
    let mw = Ident::new(DEFAULT_MIDDLEWARE, Span::call_site());

    let into_payload_impl = generate_into_payload_impl(&ident, &fields, &context, &mw, internal);
    let from_payload_impl = generate_from_payload_impl(&ident, &fields, &lifetime, &scope, &context, &mw, internal);
    let payload_impl = generate_payload_impl(&ident, &context, internal);

    let expanded = quote! {
        #into_payload_impl
        #from_payload_impl
        #payload_impl
    };

    TokenStream::from(expanded)
}

#[doc(hidden)]
fn generate_into_payload_impl(name: &Ident, fields: &Fields, context: &Ident, mw: &Ident, internal: bool) -> proc_macro2::TokenStream {
    let field_conversions = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            named.iter().enumerate().map(|(i, f)| {
                let field_name = &f.ident;
                let bit_position = i as u8;

                quote! {
                    if self.#field_name {
                        byte |= 1 << #bit_position;
                    }
                }
            }).collect::<Vec<_>>()
        },
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            unnamed.iter().enumerate().map(|(i, _)| {
                let field_name = Index::from(i);
                let bit_position = i as u8;

                quote! {
                    if self.#field_name {
                        byte |= 1 << #bit_position;
                    }
                }
            }).collect::<Vec<_>>()
        },
        Fields::Unit => vec![],
    };

    if internal {
        quote! {
            impl<#context> IntoPayload<#context> for #name {
                fn into_payload<#mw: Middleware>(&self, ctx: &mut #context, next: &mut #mw) -> Result<(), Error> {
                    let mut byte: u8 = 0;
                    #(#field_conversions)*
                    next.into_payload(&byte, ctx)
                }
            }
        }
    } else {
        quote! {
            impl<#context> npsd::IntoPayload<#context> for #name {
                fn into_payload<#mw: npsd::Middleware>(&self, ctx: &mut #context, next: &mut #mw) -> Result<(), npsd::Error> {
                    let mut byte: u8 = 0;
                    #(#field_conversions)*
                    next.into_payload(&byte, ctx)
                }
            }
        }
    }
}

#[doc(hidden)]
fn generate_from_payload_impl(name: &Ident, fields: &Fields, lifetime: &Lifetime, scope: &Lifetime, context: &Ident, mw: &Ident, internal: bool) -> proc_macro2::TokenStream {
    let field_assignments = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            named.iter().enumerate().map(|(i, f)| {
                let field_name = &f.ident;
                let bit_position = i as u8;

                quote! {
                    #field_name: (byte & (1 << #bit_position)) != 0
                }
            }).collect::<Vec<_>>()
        },
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            unnamed.iter().enumerate().map(|(i, _)| {
                let field_name = Index::from(i);
                let bit_position = i as u8;

                quote! {
                    #field_name: (byte & (1 << #bit_position)) != 0
                }
            }).collect::<Vec<_>>()
        },
        Fields::Unit => vec![],
    };

    if internal {
        quote! {
            impl<#lifetime, #context> FromPayload<#lifetime, #context> for #name {
                fn from_payload<#scope, #mw: Middleware>(ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, Error>
                    where
                        #lifetime: #scope
                {
                    let byte: u8 = next.from_payload(ctx)?;

                    Ok(#name {
                        #(#field_assignments),*
                    })
                }
            }
        }
    } else {
        quote! {
            impl<#lifetime, #context> npsd::FromPayload<#lifetime, #context> for #name {
                fn from_payload<#scope, #mw: npsd::Middleware>(ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, npsd::Error>
                    where
                        #lifetime: #scope
                {
                    let byte: u8 = next.from_payload(ctx)?;

                    Ok(#name {
                        #(#field_assignments),*
                    })
                }
            }
        }
    }
}

#[doc(hidden)]
fn generate_payload_impl(name: &Ident, context: &Ident, internal: bool) -> proc_macro2::TokenStream {
    if internal {
        quote! {
            impl<#context> Payload<#context> for #name {}
        }
    } else {
        quote! {
            impl<#context> npsd::Payload<#context> for #name {}
        }
    }
}

#[proc_macro_derive(AsyncSchema)]
pub fn async_schema_public_impl(input: TokenStream) -> TokenStream {
    async_schema_impl(input, false)
}

#[doc(hidden)]
#[proc_macro_derive(AsyncSchemaInternal)]
pub fn async_schema_internal_impl(input: TokenStream) -> TokenStream {
    async_schema_impl(input, true)
}

#[doc(hidden)]
fn async_schema_impl(input: TokenStream, internal: bool) -> TokenStream {
    let DeriveInput { ident, data, generics, .. } = parse_macro_input!(input);
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let (lifetime_exist, lifetime) = resolve_lifetime(&generics, DEFAULT_LIFETIME);
    let context = Ident::new(DEFAULT_CONTEXT, Span::call_site());
    let scope = Lifetime::new(DEFAULT_SCOPE_LIFETIME, Span::call_site());
    let mw = Ident::new(DEFAULT_MIDDLEWARE, Span::call_site());
    let mut context_generics = generics.clone();

    let context_param: GenericParam = syn::parse_quote!(#context);
    context_generics.params.push(context_param);

    let mut into_generics = context_generics.clone();
    let mut from_generics = context_generics.clone();
    let mut payload_generics = context_generics.clone();

    if !lifetime_exist {
        let lifetime_param = LifetimeParam::new(lifetime.clone());
        from_generics.params.insert(0, GenericParam::Lifetime(lifetime_param));
    }
    
    async_schema_into_impl(&mut into_generics, internal, &context);
    let (into_impl, _, _) = into_generics.split_for_impl();

    async_schema_from_impl(&mut from_generics, internal, &lifetime, &context);
    let (from_impl, _, _) = from_generics.split_for_impl();

    async_schema_payload_impl(&mut payload_generics, internal, &context);
    let (payload_impl, _, _) = payload_generics.split_for_impl();

    let sender_block = match data.clone() {
        Data::Struct(data_struct) => {
            let fields = match data_struct.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    named.iter().map(|f| {
                        let name = &f.ident;
                        let span = f.span();

                        quote_spanned! { span =>
                            next.poll_into_payload(&self.#name, ctx).await?;
                        }
                    }).collect::<Vec<_>>()
                },

                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    unnamed.iter().enumerate().map(|(i, _)| {
                        let index = Index::from(i);
                        let span = index.span();

                        quote_spanned! { span =>
                            next.poll_into_payload(&self.#index, ctx).await?;
                        }
                    }).collect::<Vec<_>>()
                },

                Fields::Unit => Vec::new(),
            };

            quote! { #( #fields )* }
        },
        Data::Enum(DataEnum { variants, .. }) => {
            let variant_cases = variants.iter().enumerate().map(|(index, variant)| {
                let variant_ident = &variant.ident;
                let variant_span = variant.span(); 

                match &variant.fields {
                    Fields::Named(FieldsNamed { named, .. }) => {
                        let (field_patterns, field_serializations): (Vec<_>, Vec<_>) = named.iter()
                            .map(|f| {
                                let name = f.ident.as_ref().unwrap();
                                let span = name.span();
                                let pattern = quote_spanned! { span => #name };
                                let serialization = quote_spanned! { span => next.poll_into_payload(&#name, ctx).await?; };
                                (pattern, serialization)
                            }).unzip();

                        quote_spanned! { variant_span => 
                            #ident::#variant_ident { #(#field_patterns,)* } => {
                                next.poll_into_payload(&#index, ctx).await?;
                                #( #field_serializations )*
                            }
                        }
                    },
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let (field_patterns, field_serializations): (Vec<_>, Vec<_>) = unnamed.iter().enumerate()
                            .map(|(i, _)| {
                                let field_name = Ident::new(&format!("__self_{}", i), Span::call_site());
                                let pattern = quote! { #field_name };
                                let serialization = quote! { next.poll_into_payload(&#field_name, ctx).await?; };
                                (pattern, serialization)
                            }).unzip();
                    
                        quote_spanned! { variant_span => 
                            #ident::#variant_ident( #( #field_patterns, )* ) => {
                                next.poll_into_payload(&#index, ctx).await?;
                                #( #field_serializations )*
                            }
                        }
                    },
                    Fields::Unit => {
                        quote_spanned! { variant_span => 
                            #ident::#variant_ident => {
                                next.poll_into_payload(&#index, ctx).await?;
                            }
                        }
                    },
                }
            });

            quote! {
                match self {
                    #( #variant_cases, )*
                }
            }
        },
        Data::Union(_) => {
            return quote! {
                compile_error!("Union types are not supported by this macro.");
            }.into();
        },
    };

    let receiver_block = match data.clone() {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    let fields = named.iter().map(|f| {
                        let field = &f.ident;
                        let ty = &f.ty;
                        let span = f.span();

                        quote_spanned! { span =>
                            #field: next.poll_from_payload::<#context, #ty>(ctx).await? // as #ty
                        }
                    }).collect::<Vec<_>>();

                    quote! {
                        Ok(#ident {
                            #( #fields ),*
                        })
                    }
                },
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let fields = unnamed.iter().enumerate().map(|(_, f)| {
                        let ty = &f.ty;

                        quote! {
                            next.poll_from_payload::<#context, #ty>(ctx).await? // as #ty
                        }
                    }).collect::<Vec<_>>();

                    quote! {
                        Ok(#ident (
                            #( #fields ),*
                        ))
                    }
                },
                Fields::Unit => {
                    quote! {
                        Ok(#ident)
                    }
                },
            }
        },
        Data::Enum(DataEnum { variants, .. }) => {
            let match_variants = variants.iter().enumerate().map(|(index, variant)| {
                let variant_ident = &variant.ident;
                
                match &variant.fields {
                    Fields::Named(FieldsNamed { named, .. }) => {
                        let deserializations = named.iter().map(|f| {
                            let name = &f.ident;
                            let ty = &f.ty;
        
                            quote! {
                                #name: next.poll_from_payload::<#context, #ty>(ctx).await? // as #ty
                            }
                        });
                        
                        quote! {
                            #index => Ok(#ident::#variant_ident { #(#deserializations),* })
                        }
                    },
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let deserializations = unnamed.iter().map(|f| {
                            let ty = &f.ty;
        
                            quote! {
                                next.poll_from_payload::<#context, #ty>(ctx).await? // as #ty
                            }
                        });
                        
                        quote! {
                            #index => Ok(#ident::#variant_ident( #(#deserializations),* ))
                        }
                    },
                    Fields::Unit => {
                        quote! {
                            #index => Ok(#ident::#variant_ident)
                        }
                    },
                }
            }).collect::<Vec<_>>();
        
            if internal {
                quote! {
                    let variant_index: usize = next.poll_from_payload(ctx).await?;
            
                    match variant_index {
                        #(#match_variants,)*
                        _ => Err(Error::UnknownVariant("Index out of bounds for enum".to_string())),
                    }
                }
            } else {
                quote! {
                    let variant_index: usize = next.poll_from_payload(ctx).await?;
            
                    match variant_index {
                        #(#match_variants,)*
                        _ => Err(npsd::Error::UnknownVariant("Index out of bounds for enum".to_string())),
                    }
                }
            }
        },
        Data::Union(_) => {
            return quote! {
                compile_error!("Union types are not supported by this macro.");
            }.into();
        },
    };

    let gen = if internal {
        quote! {
            impl #into_impl AsyncIntoPayload<#context> for #ident #ty_generics #where_clause {
                async fn poll_into_payload<#mw: AsyncMiddleware>(&self, ctx: &mut #context, next: &mut #mw) -> Result<(), Error> {
                    #sender_block
                    Ok(())
                }
            }

            impl #from_impl AsyncFromPayload<#lifetime, #context> for #ident #ty_generics #where_clause {
                async fn poll_from_payload<#scope, #mw: AsyncMiddleware>(ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, Error>
                    where
                        #lifetime: #scope
                {
                    #receiver_block
                }
            }

            impl #payload_impl AsyncPayload<#context> for #ident #ty_generics #where_clause {}
        }
    } else {
        quote! {
            impl #into_impl npsd::AsyncIntoPayload<#context> for #ident #ty_generics #where_clause {
                async fn poll_into_payload<#mw: npsd::AsyncMiddleware>(&self, ctx: &mut #context, next: &mut #mw) -> Result<(), npsd::Error> {
                    #sender_block
                    Ok(())
                }
            }

            impl #from_impl npsd::AsyncFromPayload<#lifetime, #context> for #ident #ty_generics #where_clause {
                async fn poll_from_payload<#scope, #mw: npsd::AsyncMiddleware>(ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, npsd::Error>
                    where
                        #lifetime: #scope
                {
                    #receiver_block
                }
            }

            impl #payload_impl npsd::AsyncPayload<#context> for #ident #ty_generics #where_clause {}
        }
    };

    gen.into()
}


#[proc_macro_derive(AsyncBitmap)]
pub fn async_bitmap_derive(input: TokenStream) -> TokenStream {
    async_bitmap_impl(input, false)
}

#[doc(hidden)]
#[proc_macro_derive(AsyncBitmapInternal)]
pub fn async_bitmap_internal_derive(input: TokenStream) -> TokenStream {
    async_bitmap_impl(input, true)
}

#[doc(hidden)]
fn async_bitmap_impl(input: TokenStream, internal: bool) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        Data::Struct(ref data_struct) => &data_struct.fields,
        _ => {
            return quote! {
                compile_error!("Bitmap can only be derived for structs with named or unnamed fields");
            }.into();
        } 
    };

    let field_count = match fields {
        Fields::Named(ref named_fields) => named_fields.named.len(),
        Fields::Unnamed(ref unnamed_fields) => unnamed_fields.unnamed.len(),
        Fields::Unit => 0,
    };

    if field_count > 8 {
        return quote! {
            compile_error!("Bitmap can only be derived for structs with no more than 8 fields");
        }.into();
    }

    let lifetime = Lifetime::new(DEFAULT_LIFETIME, Span::call_site());
    let scope = Lifetime::new(DEFAULT_SCOPE_LIFETIME, Span::call_site());

    let context = Ident::new(DEFAULT_CONTEXT, Span::call_site());
    let mw = Ident::new(DEFAULT_MIDDLEWARE, Span::call_site());

    let into_payload_impl = async_generate_into_payload_impl(&ident, &fields, &context, &mw, internal);
    let from_payload_impl = async_generate_from_payload_impl(&ident, &fields, &lifetime, &scope, &context, &mw, internal);
    let payload_impl = async_generate_payload_impl(&ident, &context, internal);

    let expanded = quote! {
        #into_payload_impl
        #from_payload_impl
        #payload_impl
    };

    TokenStream::from(expanded)
}

#[doc(hidden)]
fn async_generate_into_payload_impl(name: &Ident, fields: &Fields, context: &Ident, mw: &Ident, internal: bool) -> proc_macro2::TokenStream {
    let field_conversions = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            named.iter().enumerate().map(|(i, f)| {
                let field_name = &f.ident;
                let bit_position = i as u8;

                quote! {
                    if self.#field_name {
                        byte |= 1 << #bit_position;
                    }
                }
            }).collect::<Vec<_>>()
        },
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            unnamed.iter().enumerate().map(|(i, _)| {
                let field_name = Index::from(i);
                let bit_position = i as u8;

                quote! {
                    if self.#field_name {
                        byte |= 1 << #bit_position;
                    }
                }
            }).collect::<Vec<_>>()
        },
        Fields::Unit => vec![],
    };

    if internal {
        quote! {
            impl<#context> AsyncIntoPayload<#context> for #name {
                async fn poll_into_payload<#mw: AsyncMiddleware>(&self, ctx: &mut #context, next: &mut #mw) -> Result<(), Error> {
                    let mut byte: u8 = 0;
                    #(#field_conversions)*
                    next.poll_into_payload(&byte, ctx).await
                }
            }
        }
    } else {
        quote! {
            impl<#context> npsd::AsyncIntoPayload<#context> for #name {
                async fn poll_into_payload<#mw: npsd::AsyncMiddleware>(&self, ctx: &mut #context, next: &mut #mw) -> Result<(), npsd::Error> {
                    let mut byte: u8 = 0;
                    #(#field_conversions)*
                    next.poll_into_payload(&byte, ctx).await
                }
            }
        }
    }
}

#[doc(hidden)]
fn async_generate_from_payload_impl(name: &Ident, fields: &Fields, lifetime: &Lifetime, scope: &Lifetime, context: &Ident, mw: &Ident, internal: bool) -> proc_macro2::TokenStream {
    let field_assignments = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            named.iter().enumerate().map(|(i, f)| {
                let field_name = &f.ident;
                let bit_position = i as u8;

                quote! {
                    #field_name: (byte & (1 << #bit_position)) != 0
                }
            }).collect::<Vec<_>>()
        },
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            unnamed.iter().enumerate().map(|(i, _)| {
                let field_name = Index::from(i);
                let bit_position = i as u8;

                quote! {
                    #field_name: (byte & (1 << #bit_position)) != 0
                }
            }).collect::<Vec<_>>()
        },
        Fields::Unit => vec![],
    };

    if internal {
        quote! {
            impl<#lifetime, #context> AsyncFromPayload<#lifetime, #context> for #name {
                async fn poll_from_payload<#scope, #mw: AsyncMiddleware>(ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, Error>
                    where
                        #lifetime: #scope
                {
                    let byte: u8 = next.poll_from_payload(ctx).await?;

                    Ok(#name {
                        #(#field_assignments),*
                    })
                }
            }
        }
    } else {
        quote! {
            impl<#lifetime, #context> npsd::AsyncFromPayload<#lifetime, #context> for #name {
                async fn poll_from_payload<#scope, #mw: npsd::AsyncMiddleware>(ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, npsd::Error>
                    where
                        #lifetime: #scope
                {
                    let byte: u8 = next.poll_from_payload(ctx).await?;

                    Ok(#name {
                        #(#field_assignments),*
                    })
                }
            }
        }
    }
}

#[doc(hidden)]
fn async_generate_payload_impl(name: &Ident, context: &Ident, internal: bool) -> proc_macro2::TokenStream {
    if internal {
        quote! {
            impl<#context> AsyncPayload<#context> for #name {}
        }
    } else {
        quote! {
            impl<#context> npsd::AsyncPayload<#context> for #name {}
        }
    }
}