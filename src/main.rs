use quote::quote;
use syn::File;

mod generator;
mod ir;
mod parser;
mod transformer;

fn main() {
    let code = quote! {
        fn main() {
            let x = 10;
        }
    };

    let ast: File = syn::parse2(code).expect("failed to parse ast");

    for item in ast.items {
        let ir = transformer::transform_item_to_ir(&item);
        println!("{:#?}", ir);
    }
}
