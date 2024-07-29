use quote::quote;
use syn::File;

mod parser;
mod transformer;
mod generator;

fn main() {
    let code = quote! {
        fn main() {
            let x = 10;

            println!("Hello, World!");
        }
    };

    let ast: File = syn::parse2(code).expect("failed to parse ast");

    println!("{:#?}", ast);
}