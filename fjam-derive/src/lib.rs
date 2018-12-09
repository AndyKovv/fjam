extern crate syn;

#[macro_use]
extern crate quote;

extern crate proc_macro;

use syn::{parse_macro_input, parse_quote, ItemFn};
use proc_macro::TokenStream;

fn create_content_block(block: Box<syn::Block>) -> syn::Stmt {
    // Method created context block with catch_unwind
    // Reason why it creates test body with catch_unwind
    // is raise panic after tear_down complete.
    let wrap_body = quote!{
        let result = ::std::panic::catch_unwind(|| {
            #block;
        });
    };
    let stmt: syn::Stmt = parse_quote! {
        #wrap_body
    };
    stmt
}

fn create_setup_call() -> syn::Stmt {
    // Method creates set_up hook
    let set_up_call = quote!(set_up(););
    let stmt: syn::Stmt = parse_quote! {
        #set_up_call
    };
    stmt
}

fn create_tear_down_call() -> syn::Stmt {
    // Method creates tear_down hook
    let tear_down_call = quote!(tear_down(););
    let stmt: syn::Stmt = parse_quote! {
        #tear_down_call
    };
    stmt
}

fn create_result_check() -> syn::Stmt {
    // Method should create result check
    let result_check = quote!{
        if let Err(err) = result {
            ::std::panic::resume_unwind(err);
        };
    };
    let stmt: syn::Stmt = parse_quote! {
        #result_check
    };
    stmt
}

/// Integration test macros
/// Usage:
/// If you want add hooks to you integaration test,
/// which need to create something before test, or remove something after test
/// need add #[integration_test] macro after #[test] macro.
/// If you add #[integration_test] you must add set_up(), and tear_down() hooks
///
/// Example:
///
/// #[cfg(test)]
/// pub mod some_test {
///    use super::*;
///
///  fn set_up() {
///      println!("Set up");
///  }
///  fn tear_down() {
///      println!("Tear down");
///  }
///
///  #[test]
///  #[integration_test]
///  fn my_shiney_integration_test() {
///      asserteq!(1, 2);
///  }
/// }
/// Output order is:
///  "Set up",
///  panic!() -> is going to show after tear_down finish
///  "Tear down"

#[proc_macro_attribute]
pub fn integration_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    /* Macros add tear_down and set_up hooks to choosen test 
       TODO Add unittests!!!!!
       unitests should have state for control execution
    */

    let mut input_test_function = parse_macro_input!(input as ItemFn);
    // TODO, at next iteration need add manager for down methods
    // this should be structure IntegrationTest, with exists logic
    // for maintaine it in easy way
    let block = input_test_function.block.clone();
    let content_block = create_content_block(block);
    let setup_call = create_setup_call();
    let tear_down_call = create_tear_down_call();
    let result_check = create_result_check();

    let call_vector: Vec<syn::Stmt> = vec![
        setup_call, content_block,
        tear_down_call, result_check
    ];

    input_test_function.block.stmts = call_vector;
    quote!(#input_test_function).to_string().parse().unwrap()
}
