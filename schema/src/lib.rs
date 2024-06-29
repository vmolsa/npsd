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
fn add_lifetime_to_generics(generics: &mut Generics, lifetime_name: &str) -> Lifetime {
    if let Some(existing_lifetime) = generics.params.iter().find_map(|param| {
        if let GenericParam::Lifetime(lifetime_param) = param {
            Some(lifetime_param.lifetime.clone())
        } else {
            None
        }
    }) {
        return existing_lifetime;
    }

    let lifetime_ident = Lifetime::new(lifetime_name, Span::call_site());
    let lifetime_param = LifetimeParam::new(lifetime_ident.clone());

    generics.params.insert(0, GenericParam::Lifetime(lifetime_param));
    lifetime_ident
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
fn check_default_bounds(generics: &mut Generics, internal: bool, lifetime: &Lifetime, context: &Ident) {
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
            if !has_bound(&type_param.bounds, "FromPayload") {
                type_param.bounds.push(if internal {
                    parse_quote!(FromPayload<#lifetime, #context>)
                } else {
                    parse_quote!(npsd::FromPayload<#lifetime, #context>)
                });
            }
            if !has_bound(&type_param.bounds, "Payload") {
                type_param.bounds.push(if internal {
                    parse_quote!(Payload<#lifetime, #context>)
                } else {
                    parse_quote!(npsd::Payload<#lifetime, #context>)
                });
            }
            if !has_bound(&type_param.bounds, "PayloadInfo") {
                type_param.bounds.push(if internal {
                    parse_quote!(PayloadInfo)
                } else {
                    parse_quote!(npsd::PayloadInfo)
                });
            }
        }
    }
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
    let (impl_generics_basic, ty_generics, where_clause) = generics.split_for_impl();
    let mut impl_generics = generics.clone();

    let lifetime = add_lifetime_to_generics(&mut impl_generics, DEFAULT_LIFETIME);
    let scope = Lifetime::new(DEFAULT_SCOPE_LIFETIME, Span::call_site());
    let context = Ident::new(DEFAULT_CONTEXT, Span::call_site());
    let mw = Ident::new(DEFAULT_MIDDLEWARE, Span::call_site());

    let context_param: GenericParam = if internal {
        syn::parse_quote!(#context: PayloadContext)
    } else {
        syn::parse_quote!(#context: npsd::PayloadContext)
    };

    impl_generics.params.push(context_param);

    check_default_bounds(&mut impl_generics, internal, &lifetime, &context);

    let (impl_generics, _, _) = impl_generics.split_for_impl();

    let sender_block = match data.clone() {
        Data::Struct(data_struct) => {
            let fields = match data_struct.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    named.iter().map(|f| {
                        let name = &f.ident;
                        let span = f.span();

                        quote_spanned! { span =>
                            next.into_payload(&self.#name, handler, ctx)?;
                        }
                    }).collect::<Vec<_>>()
                },

                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    unnamed.iter().enumerate().map(|(i, _)| {
                        let index = Index::from(i);
                        let span = index.span();

                        quote_spanned! { span =>
                            next.into_payload(&self.#index, handler, ctx)?;
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
                                let serialization = quote_spanned! { span => next.into_payload(&#name, handler, ctx)?; };
                                (pattern, serialization)
                            }).unzip();

                        quote_spanned! { variant_span => 
                            #ident::#variant_ident { #(#field_patterns,)* } => {
                                next.into_payload(&#index, handler, ctx)?;
                                #( #field_serializations )*
                            }
                        }
                    },
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let (field_patterns, field_serializations): (Vec<_>, Vec<_>) = unnamed.iter().enumerate()
                            .map(|(i, _)| {
                                let field_name = Ident::new(&format!("__self_{}", i), Span::call_site());
                                let pattern = quote! { #field_name };
                                let serialization = quote! { next.into_payload(&#field_name, handler, ctx)?; };
                                (pattern, serialization)
                            }).unzip();
                    
                        quote_spanned! { variant_span => 
                            #ident::#variant_ident( #( #field_patterns, )* ) => {
                                next.into_payload(&#index, handler, ctx)?;
                                #( #field_serializations )*
                            }
                        }
                    },
                    Fields::Unit => {
                        quote_spanned! { variant_span => 
                            #ident::#variant_ident => {
                                next.into_payload(&#index, handler, ctx)?;
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
                            #field: next.from_payload::<#context, #ty>(handler, ctx)? // as #ty
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
                            next.from_payload::<#context, #ty>(handler, ctx)? // as #ty
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
                                #name: next.from_payload::<#context, #ty>(handler, ctx)? // as #ty
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
                                next.from_payload::<#context, #ty>(handler, ctx)? // as #ty
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
                    let variant_index: usize = next.from_payload(handler, ctx)?;
            
                    match variant_index {
                        #(#match_variants,)*
                        _ => Err(Error::UnknownVariant("Index out of bounds for enum".to_string())),
                    }
                }
            } else {
                quote! {
                    let variant_index: usize = next.from_payload(handler, ctx)?;
            
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
            impl #impl_generics IntoPayload<#context> for #ident #ty_generics #where_clause {
                fn into_payload<#mw: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut #context, next: &mut #mw) -> Result<(), Error> {
                    #sender_block
                    Ok(())
                }
            }

            impl #impl_generics FromPayload<#lifetime, #context> for #ident #ty_generics #where_clause {
                fn from_payload<#scope, #mw: Middleware>(handler: &#scope mut PayloadHandler<#lifetime>, ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, Error>
                    where
                        #lifetime: #scope
                {
                    #receiver_block
                }
            }

            impl #impl_generics Payload<#lifetime, #context> for #ident #ty_generics #where_clause {}

            impl #impl_generics_basic PayloadInfo for #ident #ty_generics #where_clause {
                const HASH: u64 = PayloadConstHash(stringify!(#ident).as_bytes());
                const TYPE: &'static str = stringify!(#ident);
            }
        }
    } else {
        quote! {
            impl #impl_generics npsd::IntoPayload<#context> for #ident #ty_generics #where_clause {
                fn into_payload<#mw: npsd::Middleware>(&self, handler: &mut npsd::PayloadHandler<'_>, ctx: &mut #context, next: &mut #mw) -> Result<(), npsd::Error> {
                    #sender_block
                    Ok(())
                }
            }

            impl #impl_generics npsd::FromPayload<#lifetime, #context> for #ident #ty_generics #where_clause {
                fn from_payload<#scope, #mw: npsd::Middleware>(handler: &#scope mut npsd::PayloadHandler<#lifetime>, ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, npsd::Error>
                    where
                        #lifetime: #scope
                {
                    #receiver_block
                }
            }

            impl #impl_generics npsd::Payload<#lifetime, #context> for #ident #ty_generics #where_clause {}

            impl #impl_generics_basic npsd::PayloadInfo for #ident #ty_generics #where_clause {
                const HASH: u64 = npsd::PayloadConstHash(stringify!(#ident).as_bytes());
                const TYPE: &'static str = stringify!(#ident);
            }
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
    let payload_impl = generate_payload_impl(&ident, &lifetime, &context, internal);

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
            impl<#context: PayloadContext> IntoPayload<#context> for #name {
                fn into_payload<#mw: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut #context, next: &mut #mw) -> Result<(), Error> {
                    let mut byte: u8 = 0;
                    #(#field_conversions)*
                    next.into_payload(&byte, handler, ctx)
                }
            }
        }
    } else {
        quote! {
            impl<#context: npsd::PayloadContext> npsd::IntoPayload<#context> for #name {
                fn into_payload<#mw: npsd::Middleware>(&self, handler: &mut npsd::PayloadHandler<'_>, ctx: &mut #context, next: &mut #mw) -> Result<(), npsd::Error> {
                    let mut byte: u8 = 0;
                    #(#field_conversions)*
                    next.into_payload(&byte, handler, ctx)
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
            impl<#lifetime, #context: PayloadContext> FromPayload<#lifetime, #context> for #name {
                fn from_payload<#scope, #mw: Middleware>(handler: &#scope mut PayloadHandler<#lifetime>, ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, Error>
                    where
                        #lifetime: #scope
                {
                    let byte: u8 = next.from_payload(handler, ctx)?;

                    Ok(#name {
                        #(#field_assignments),*
                    })
                }
            }
        }
    } else {
        quote! {
            impl<#lifetime, #context: npsd::PayloadContext> npsd::FromPayload<#lifetime, #context> for #name {
                fn from_payload<#scope, #mw: npsd::Middleware>(handler: &#scope mut npsd::PayloadHandler<#lifetime>, ctx: &mut #context, next: &#scope mut #mw) -> Result<Self, npsd::Error>
                    where
                        #lifetime: #scope
                {
                    let byte: u8 = next.from_payload(handler, ctx)?;

                    Ok(#name {
                        #(#field_assignments),*
                    })
                }
            }
        }
    }
}

#[doc(hidden)]
fn generate_payload_impl(name: &Ident, lifetime: &Lifetime, context: &Ident, internal: bool) -> proc_macro2::TokenStream {
    if internal {
        quote! {
            impl<#lifetime, #context: PayloadContext> Payload<#lifetime, #context> for #name {}

            impl PayloadInfo for #name {
                const HASH: u64 = PayloadConstHash(stringify!(#name).as_bytes());
                const TYPE: &'static str = stringify!(#name);
            }
        }
    } else {
        quote! {
            impl<#lifetime, #context: npsd::PayloadContext> npsd::Payload<#lifetime, #context> for #name {}

            impl npsd::PayloadInfo for #name {
                const HASH: u64 = npsd::PayloadConstHash(stringify!(#name).as_bytes());
                const TYPE: &'static str = stringify!(#name);
            }
        }
    }
}