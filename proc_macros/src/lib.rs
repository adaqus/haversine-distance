use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn profile(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let sig = &input.sig;
    let fn_name = &sig.ident;
    let block = &input.block;

    let expanded = quote! {
        #sig {
            let start = self::profiler::read_tsc();
            
            let result = (|| #block)();
            
            let end = self::profiler::read_tsc();
            
            self::profiler::store_profile_data(stringify!(#fn_name).to_string(), end.wrapping_sub(start));
            
            result
        }
    };

    TokenStream::from(expanded)
}
