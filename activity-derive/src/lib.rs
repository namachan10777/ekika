use proc_macro::TokenStream;

#[proc_macro]
pub fn derive_activity(_item: TokenStream) -> TokenStream {
    "// unimplmented!".parse().unwrap()
}
