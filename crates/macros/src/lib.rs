use macro_magic::import_tokens_attr;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

fn derive_tessellatable_internal(item: TokenStream2) -> syn::Result<TokenStream2> {
    let item = syn::parse2::<syn::ItemEnum>(item)?;

    let name = item.ident.clone();
    let node_names = item
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();

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
    let node_names = item
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();
    let nodes = item.variants.iter().map(|variant| {
        let fields = variant.fields.iter();
        quote! {
            #(#fields)*
        }
    });

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

        #(
            impl From<#nodes> for #name {
                fn from(node: #nodes) -> Self {
                    Self::#node_names(node)
                }
            }
        )*
    };
    Ok(res)
}

fn derive_component_provider_internal(item: TokenStream2) -> syn::Result<TokenStream2> {
    let item = syn::parse2::<syn::ItemEnum>(item)?;

    let name = item.ident.clone();
    let node_names = item
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();

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

fn combine_enum_internal(
    attr: TokenStream2,
    item: TokenStream2,
    foreign_path: syn::Path,
) -> syn::Result<TokenStream2> {
    let mut local_enum = syn::parse2::<syn::ItemEnum>(item.clone())?;
    let local_name = local_enum.ident.clone();

    let foreign_enum = syn::parse2::<syn::ItemEnum>(attr)?;
    let foreign_variants = foreign_enum
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();
    let foreign_args = foreign_enum
        .variants
        .iter()
        .map(|variant| {
            variant
                .fields
                .iter()
                .map(|field| field.ident.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    foreign_enum.variants.iter().for_each(|variant| {
        if local_enum
            .variants
            .iter()
            .any(|local_variant| local_variant.ident == variant.ident)
        {
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

fn nested_message_internal(item: TokenStream2) -> syn::Result<TokenStream2> {
    let item = syn::parse2::<syn::ItemEnum>(item)?;

    let name = item.ident.clone();
    let message_names = item
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();
    let messages = item
        .variants
        .iter()
        .map(|variant| {
            let fields = variant.fields.iter();
            quote! {
                #(#fields)*
            }
        })
        .collect::<Vec<_>>();

    let res = quote! {
        #item

        #(
            impl From<#messages> for #name {
                fn from(message: #messages) -> Self {
                    Self::#message_names(message)
                }
            }
        )*

        #(
            impl TryFrom<#name> for #messages {
                type Error = ();

                fn try_from(message: #name) -> Result<Self, Self::Error> {
                    match message {
                        #name::#message_names(message) => Ok(message),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
    Ok(res)
}

fn combine_response_internal(
    attr: TokenStream2,
    item: TokenStream2,
    foreign_path: syn::Path,
) -> syn::Result<TokenStream2> {
    let mut m_replacement: Option<TokenStream2> = None;
    let mut n_replacement: Option<TokenStream2> = None;

    let path = foreign_path.segments.last().unwrap().clone();
    if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        args, ..
    }) = path.arguments
    {
        let mut iter = args.iter();
        if let Some(arg1) = iter.next() {
            if let syn::GenericArgument::Type(ty) = arg1 {
                m_replacement = Some(quote!(#ty));
            }
        }
        if let Some(arg2) = iter.next() {
            if let syn::GenericArgument::Type(ty) = arg2 {
                n_replacement = Some(quote!(#ty));
            }
        }
    }

    let mut local_enum = syn::parse2::<syn::ItemEnum>(item.clone())?;
    let local_name = local_enum.ident.clone();

    let foreign_enum = syn::parse2::<syn::ItemEnum>(attr)?;
    let foreign_name = foreign_enum.ident.clone();

    let foreign_variants = foreign_enum
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();

    let foreign_args = foreign_enum
        .variants
        .iter()
        .map(|variant| {
            variant
                .fields
                .iter()
                .map(|field| field.ident.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    foreign_enum.variants.iter().for_each(|variant| {
        if local_enum
            .variants
            .iter()
            .any(|local_variant| local_variant.ident == variant.ident)
        {
            return;
        }
        let mut variant = variant.clone();
        for field in variant.fields.iter_mut() {
            match &(field.ty.to_token_stream().to_string())[..] {
                "M" => {
                    field.ty = syn::parse2::<syn::Type>(quote! { #m_replacement }).unwrap();
                }
                "N" => {
                    field.ty = syn::parse2::<syn::Type>(quote! { #n_replacement }).unwrap();
                }
                _ => {}
            }
        }
        local_enum.variants.push(variant);
    });

    let res = quote! {
        #local_enum

        impl From<#foreign_path> for #local_name {
            fn from(response: #foreign_path) -> Self {
                match response {
                    #(
                        #foreign_name::#foreign_variants { #(#foreign_args,)* } => Self::#foreign_variants { #(#foreign_args,)* },
                    )*
                }
            }
        }

        impl TryInto<#foreign_path> for #local_name {
            type Error = ();

            fn try_into(self) -> Result<#foreign_path, Self::Error> {
                match self {
                    #(
                        Self::#foreign_variants { #(#foreign_args,)* } => Ok(#foreign_name::#foreign_variants { #(#foreign_args,)* }),
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

#[proc_macro_attribute]
pub fn nested_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let res = match nested_message_internal(item.into()) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    };
    res.into()
}

#[import_tokens_attr]
#[proc_macro_attribute]
pub fn combine_response(attr: TokenStream, item: TokenStream) -> TokenStream {
    let foreign_path = syn::parse::<syn::Path>(__source_path).unwrap();
    let res = match combine_response_internal(attr.into(), item.into(), foreign_path) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    };
    res.into()
}

#[proc_macro_attribute]
pub fn radiant_wasm_bindgen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = syn::parse2::<syn::Item>(item.into()).unwrap();

    #[cfg(target_arch = "wasm32")]
    let res = quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        #item
    };

    #[cfg(not(arget_arch = "wasm32"))]
    {
        match &mut item {
            syn::Item::Struct(syn::ItemStruct { fields, .. }) => {
                for field in fields.iter_mut() {
                    if let Some(index) = field.attrs.iter().position(|attr| {
                        match attr.meta {
                            syn::Meta::List(ref meta_list) => {
                                meta_list.path.segments.iter().any(
                                    |segment| segment.ident == "wasm_bindgen"
                                )
                            }
                            _ => false,
                        }
                    }) {
                        field.attrs.remove(index);
                    }
                }
            }
            _ => {}
        }
    }

    #[cfg(not(arget_arch = "wasm32"))]
    let res = quote! {
        #item
    };

    res.into()
}
