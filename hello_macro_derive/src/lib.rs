extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::{self, Data};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast: DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_hello_macro(ast)
}

fn impl_hello_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = ast.ident;
    let gen_1 = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen_1.into()
}

#[proc_macro_derive(MyDefault_1)]
pub fn my_default(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = ast.ident;

    let Data::Struct(s) = ast.data else {
        panic!("MyDefault dervive must use in struct");
    };

    let mut field_ast = quote!();

    for (idx, field) in s.fields.into_iter().enumerate() {
        let (field_id, field_type) = (&field.ident, &field.ty);

        // 匿名字段
        if field_id.is_none() {
            let field_idx = syn::Index::from(idx);
            field_ast.extend(quote! {
                #field_idx: #field_type::default(),
            });
        } else {
            field_ast.extend(quote! {
                #field_id: #field_type::default(),
            });
        }
    }
    quote! {
        impl Default for # id {
            fn default() -> Self {
                Self {
                    #field_ast
                }
            }
        }
    }
    .into()
}
