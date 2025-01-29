#![feature(let_chains, stmt_expr_attributes)]
#![allow(soft_unstable)]
#![cfg_attr(test, feature(test))]
mod compose_macro;
mod argument;
pub mod components;
pub use components::Component;

#[cfg(test)]
mod tests;
