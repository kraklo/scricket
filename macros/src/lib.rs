use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(AsComponentEvent)]
pub fn as_component_event_derive(_input: TokenStream) -> TokenStream {
    let _ast: syn::DeriveInput = syn::parse(_input).unwrap();

    if let syn::Data::Enum(_event) = _ast.data {
        let _name = _ast.ident;
        let _gen = quote! {
            impl AsEvent for #_name {
                fn as_event(self) -> Event {
                    Event::ComponentEvent(ComponentEvent::#_name(self))
                }
            }
        };
        return _gen.into();
    }

    TokenStream::new()
}

#[proc_macro_derive(AsSubcomponentEvent)]
pub fn as_subcomponent_event_derive(_input: TokenStream) -> TokenStream {
    let _ast: syn::DeriveInput = syn::parse(_input).unwrap();

    if let syn::Data::Enum(_event) = _ast.data {
        let _name = _ast.ident;
        let _gen = quote! {
            impl AsEvent for #_name {
                fn as_event(self) -> Event {
                    Event::ComponentEvent(ComponentEvent::SubcomponentEvent(SubcomponentEvent::#_name(self)))
                }
            }
        };
        return _gen.into();
    }

    TokenStream::new()
}
