//! Macros used by TheK.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn;
use syn::visit::{self, Visit};
use walkdir::WalkDir;
use std::fs::File;
use std::io::Read;

struct FnVisitor {
    pub functions: Vec<(syn::ItemFn, String)>
}

impl FnVisitor {
    pub const fn new() -> Self {
        Self {
            functions: vec!()
        }
    }
} 

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        for attr in &node.attrs {
            if let Some(ident) = attr.path.get_ident() {
                if ident.to_string() == "device" {
                    let tokens = attr.tokens.clone();
                    let path = proc_macro::TokenStream::from(tokens as proc_macro2::TokenStream);
                    let path = path.to_string()
                        .replace(" ", "")
                        .replace("(", "")
                        .replace(")", "");
                    syn::parse_str::<syn::Path>(&path)
                        .expect("Debug argument must be a path");
                    self.functions.push((node.clone(), path));
                }
            }
        }

        // Delegate to the default impl to visit any nested functions.
        visit::visit_item_fn(self, node);
    }
}

#[proc_macro_attribute]
pub fn register_devices(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as syn::ExprLit);
    let ast = syn::parse_macro_input!(item as syn::ItemFn);

    impl_register_devices_macro(&ast, &attr_ast)
}

fn impl_register_devices_macro(ast: &syn::ItemFn, attr_ast: &syn::ExprLit) -> TokenStream {
    let name = &ast.sig.ident;
    let path = &attr_ast.lit;

    // Generate a list of .rs files
    let mut files = Vec::new();
    if let syn::Lit::Str(lit_str) = path {
        for entry in WalkDir::new(lit_str.value()).into_iter().filter_map(|e| e.ok()) {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".rs") {
                    files.push(entry.path().display().to_string());
                }
            }
        }
    }
    else {
        panic!("register_devices argument must be a string");
    }

    let mut visitor = FnVisitor::new();

    // For each file, parse it and find function definitions labeled with the macro "device".
    for f in files {
        // Parse rs file
        let mut file = File::open(f).expect("Failed opening file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed reading file");
        let ast = syn::parse_file(&content).expect("Failed parsing rs file");

        // Recursively visit all functions
        visitor.visit_file(&ast);
    }

    // Generate a function call for each funtion found.
    let mut calls = vec!();
    for (fn_item, path) in visitor.functions {
        let line = format!("{}::{}();", path, fn_item.sig.ident);
        calls.push(line);
    }

    let calls = calls.join("\n");
    let calls_tokens: proc_macro2::TokenStream = calls.parse().unwrap();

    // Generate the function to register all devices.
    let gen = quote! {
        /// Register all devices.
        pub fn #name() {
            #calls_tokens
        }
    };

    gen.into()
}

/// A simple pass-through macro to mark device registering functions.
#[proc_macro_attribute]
pub fn device(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
