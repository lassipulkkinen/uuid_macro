extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use uuid::Uuid;

#[proc_macro_hack]
#[doc(hidden)]
pub fn uuid_v4(_: TokenStream) -> TokenStream {
    let (
        d1,
        d2,
        d3,
        &[
            d40,
            d41,
            d42,
            d43,
            d44,
            d45,
            d46,
            d47,
        ],
    ) = Uuid::new_v4().as_fields();

    let result = quote! {
        ::uuid::Uuid::from_fields(
            #d1,
            #d2,
            #d3,
            &[
                #d40,
                #d41,
                #d42,
                #d43,
                #d44,
                #d45,
                #d46,
                #d47,
            ],
        ).unwrap()
    };

    result.into()
}
