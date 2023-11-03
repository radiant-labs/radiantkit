use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

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