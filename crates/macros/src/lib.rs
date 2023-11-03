use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use macro_magic::import_tokens_attr;

fn derive_tessellatable_internal(item: TokenStream2) -> syn::Result<TokenStream2> {
    let item = syn::parse2::<syn::ItemEnum>(item)?;

    let name = item.ident.clone();
    let node_names = item.variants.iter().map(|variant| {
        variant.ident.clone()
    }).collect::<Vec<_>>();

    let res = quote! {
        impl RadiantTessellatable for #name {
            fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
                match self {
                    #(
                        #name::#node_names(node) => node.attach(screen_descriptor),
                    )*
                }
            }

            fn detach(&mut self) {
                match self {
                    #(
                        #name::#node_names(node) => node.detach(),
                    )*
                }
            }

            fn set_needs_tessellation(&mut self) {
                match self {
                    #(
                        #name::#node_names(node) => node.set_needs_tessellation(),
                    )*
                }
            }

            fn tessellate(
                &mut self,
                selection: bool,
                screen_descriptor: &ScreenDescriptor,
                fonts_manager: &epaint::text::Fonts,
            ) -> Vec<ClippedPrimitive> {
                match self {
                    #(
                        #name::#node_names(node) => node.tessellate(selection, screen_descriptor, fonts_manager),
                    )*
                }
            }
        }
    };
    Ok(res)
}

fn derive_node_internal(item: TokenStream2) -> syn::Result<TokenStream2> {
    let item = syn::parse2::<syn::ItemEnum>(item)?;

    let name = item.ident.clone();
    let node_names = item.variants.iter().map(|variant| {
        variant.ident.clone()
    }).collect::<Vec<_>>();

    let res = quote! {
        impl RadiantNode for #name {
            fn get_id(&self) -> u64 {
                match self {
                    #(
                        #name::#node_names(node) => node.get_id(),
                    )*
                }
            }

            fn set_id(&mut self, id: u64) {
                match self {
                    #(
                        #name::#node_names(node) => node.set_id(id),
                    )*
                }
            }

            fn get_bounding_rect(&self) -> [f32; 4] {
                match self {
                    #(
                        #name::#node_names(node) => node.get_bounding_rect(),
                    )*
                }
            }
        }
    };
    Ok(res)
}

fn derive_component_provider_internal(item: TokenStream2) -> syn::Result<TokenStream2> {
    let item = syn::parse2::<syn::ItemEnum>(item)?;

    let name = item.ident.clone();
    let node_names = item.variants.iter().map(|variant| {
        variant.ident.clone()
    }).collect::<Vec<_>>();

    let res = quote! {
        impl RadiantComponentProvider for #name {
            fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T> {
                match self {
                    #(
                        #name::#node_names(node) => node.get_component(),
                    )*
                }
            }

            fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
                match self {
                    #(
                        #name::#node_names(node) => node.get_component_mut(),
                    )*
                }
            }
        }
    };
    Ok(res)
}

fn combine_enum_internal(attr: TokenStream2, item: TokenStream2, foreign_path: syn::Path) -> syn::Result<TokenStream2> {
    let mut local_enum = syn::parse2::<syn::ItemEnum>(item.clone())?;
    let local_name = local_enum.ident.clone();

    let foreign_enum = syn::parse2::<syn::ItemEnum>(attr)?;
    let foreign_variants = foreign_enum.variants.iter().map(|variant| {
        variant.ident.clone()
    }).collect::<Vec<_>>();
    let foreign_args = foreign_enum.variants.iter().map(|variant| {
        variant.fields.iter().map(|field| {
            field.ident.clone()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    foreign_enum.variants.iter().for_each(|variant| {
        if local_enum.variants.iter().any(|local_variant| local_variant.ident == variant.ident) {
            return;
        }
        local_enum.variants.push(variant.clone());
    });

    let res = quote! {
        #local_enum

        impl From<#foreign_path> for #local_name {
            fn from(foreign: #foreign_path) -> Self {
                match foreign {
                    #(
                        #foreign_path::#foreign_variants { #(#foreign_args,)* } => Self::#foreign_variants { #(#foreign_args,)* },
                    )*
                }
            }
        }

        impl TryInto<#foreign_path> for #local_name {
            type Error = ();
        
            fn try_into(self) -> Result<#foreign_path, Self::Error> {
                match self {
                    #(
                        Self::#foreign_variants { #(#foreign_args,)* } => Ok(#foreign_path::#foreign_variants { #(#foreign_args,)* }),
                    )*
                    _ => Err(()),
                }
            }
        }
    };

    // use proc_utils::*;
    // res.pretty_print();

    Ok(res)
}

#[proc_macro_derive(RadiantTessellatable)]
pub fn derive_tessellatable(item: TokenStream) -> TokenStream {
    let res = match derive_tessellatable_internal(item.into()) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    };
    res.into()
}

#[proc_macro_derive(RadiantNode)]
pub fn derive_node(item: TokenStream) -> TokenStream {
    let res = match derive_node_internal(item.into()) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    };
    res.into()
}

#[proc_macro_derive(RadiantComponentProvider)]
pub fn derive_component_provider(item: TokenStream) -> TokenStream {
    let res = match derive_component_provider_internal(item.into()) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    };
    res.into()
}

#[import_tokens_attr]
#[proc_macro_attribute]
pub fn combine_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let foreign_path = syn::parse::<syn::Path>(__source_path).unwrap();
    let res = match combine_enum_internal(attr.into(), item.into(), foreign_path) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    };
    res.into()
}