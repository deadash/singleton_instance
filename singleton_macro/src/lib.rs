// Include the required crates for procedural macro.
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

/// Derive macro for creating a singleton instance of a struct.
///
/// This macro implements a function `instance()` for the struct,
/// which returns a mutable reference to a singleton instance of the struct.
/// The struct should implement the `Initializable` trait to provide
/// custom initialization.
#[proc_macro_derive(Instance)]
pub fn instance_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_instance_macro(&ast)
}

/// Internal function to implement the `instance` function for the struct.
///
/// This function generates the implementation of the `instance` function,
/// which returns a singleton instance of the struct.
fn impl_instance_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Use std::sync::Once to ensure thread-safe initialization.
    let gen = quote! {
        static mut INSTANCE: Option<#name> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        impl #name {
            /// Retrieve the singleton instance of this struct.
            ///
            /// # Returns
            /// A mutable reference to the singleton instance.
            pub fn instance() -> &'static mut #name {
                unsafe {
                    INIT.call_once(|| {
                        let s = #name::initialize();
                        INSTANCE = Some(s);
                    });
                    INSTANCE.as_mut().unwrap()
                }
            }
        }
    };
    gen.into()
}
