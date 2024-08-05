use quote::quote;
use syn::File;

mod luau;
mod transformer;

fn main() {
    let code = quote! {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    };

    let ast: File = syn::parse2(code).expect("failed to parse source ast");
    let out = transformer::transform_file_to_luau(&ast);

    println!("{:#?}", out);
}
