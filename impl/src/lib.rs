extern crate proc_macro;
extern crate quote;

use std::path::Path;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn bar(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    #[rustfmt::skip]
    let cpp =
        r#"
#include <foo.h>

extern "C" int _foo()
{{
    return foo();
}}
"#;

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let cpp_path = out_dir.join("foo.cpp");
    std::fs::write(&cpp_path, &cpp).unwrap();

    cc::Build::new()
        .cpp(true)
        .file(&cpp_path)
        .include(&out_dir)
        .compile("foo");

    return quote! {
        extern "C" {
            fn _foo() -> i32;
        }

        fn foo() -> i32 {
            unsafe { _foo() }
        }
    }
    .into();
}
