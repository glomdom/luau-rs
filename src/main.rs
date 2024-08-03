use quote::quote;
use syn::File;

mod generator;
mod luau;
mod parser;
mod transformer;

fn main() {
    let code = quote! {
        fn main(a: i32, b: String, c: f64) -> i32 {
            let x = 10;

            sigma(10);
        }

        fn sigma(a: &i32) -> i32 {
            a * 2
        }
    };

    let ast: File = syn::parse2(code).expect("failed to parse source ast");
    for item in ast.items {
        let luau = transformer::transform_item_to_luau(&item);
        println!("{:#?}", luau);
    }
}
