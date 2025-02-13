/*use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use syn::{self};

use itertools::{iproduct, Itertools};
use optsort::{
    partial_order::{PartialOrder},
    permutation::Permutation,
};

struct TreeBuilder {
    visited: Vec<PartialOrder>,
}

impl TreeBuilder {
    fn lookup_history(&self, node: &PartialOrder) -> Option<Permutation> {
        for prev in &self.visited {
            match PartialOrder::isomorphism(node, prev) {
                Some(perm) => return Some(perm),
                None => (),
            }
        }
        None
    }

    fn dfs(&mut self, node: &PartialOrder) -> TokenStream {
        if node.count_orders() == 1 {
            let elems: Vec<_> = node.get_order().iter().map(|i| quote! { a[p[#i]] }).collect();
            return quote! {
                return [#(#elems), *];
            };
        }

        let n = node.size();
        for (i, j) in iproduct!(0..n, 0..n).filter(|(i, j)| i != j) {
            match (node.with_edge_opt(i, j), node.with_edge_opt(j, i)) {
                (Some(left), Some(right))
                    if left.count_orders().abs_diff(right.count_orders()) <= 1 =>
                {
                    if let Some(perm) = isomorphism(&left, &right) {
                        let right_child = self.dfs(&right);
                        let map: Vec<_> = perm.mapping.iter().map(|i| quote! { p[#i] }).collect();
                        return quote! {
                            if a[p[#i]] > a[p[#j]] { // reverse order
                                // saving perm
                                p = [#(#map), *];
                            }
                            #right_child
                        };
                    } else {
                        let left_child = self.dfs(&left);
                        let right_child = self.dfs(&right);
                        return quote! {
                            if a[p[#i]] < a[p[#j]] {
                                #right_child
                            } else {
                                #left_child
                            }
                        };
                    }
                }
                _ => (),
            };
        }
        return quote! {};
    }

    pub fn new() -> Self {
        Self {
            _visited: Vec::new(),
        }
    }

    pub fn build(&mut self, n: usize) -> TokenStream {
        let root = PartialOrder::empty(n);
        let _identity = Permutation::identity(n);

        // self.visited.clear();
        let call = self.dfs(&root);

        let func_name = format_ident!("optsort{}", n);
        let range = 0..n;
        let prog = quote! {
            pub fn #func_name(a: [usize; #n]) -> [usize; #n] {
                let mut p = [#(#range), *];
                #call
            }
        };
        return prog;
    }
}

fn generate(n: usize) -> String {
    let mut builder = TreeBuilder::new();
    let prog = builder.build(n);

    println!("{}", prog.to_string());

    let syntax_tree = syn::parse2::<syn::File>(prog).expect("Failed to parse optsort");
    let formatted = prettyplease::unparse(&syntax_tree);
    return formatted;
}

fn main() {
    let code = (2..=7).map(|n: usize| generate(n)).join("\n");
    fs::write("src/output/optsort.rs", code).unwrap();
}
*/

fn main() {
    println!("Hello from generate");
}