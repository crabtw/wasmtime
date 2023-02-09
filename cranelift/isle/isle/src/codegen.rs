//! Generate Rust code from a series of Sequences.

use crate::sema::{TermEnv, TermId, TypeEnv};
use crate::trie_again::{BindingId, RuleSet};
use crate::StableSet;
use std::fmt::Write;

/// Options for code generation.
#[derive(Clone, Debug, Default)]
pub struct CodegenOptions {
    /// Do not include the `#![allow(...)]` pragmas in the generated
    /// source. Useful if it must be include!()'d elsewhere.
    pub exclude_global_allow_pragmas: bool,
    /// Language to emit
    pub lang: Lang,
}

/// Emitted language
#[derive(Copy, Clone, Debug, Default)]
pub enum Lang {
    /// Rust
    #[default]
    Rust,
    /// C++
    Cxx,
}

/// Emit Rust source code for the given type and term environments.
pub fn codegen(
    typeenv: &TypeEnv,
    termenv: &TermEnv,
    terms: &[(TermId, RuleSet)],
    options: &CodegenOptions,
) -> String {
    match options.lang {
        Lang::Rust => rust::Codegen::compile(typeenv, termenv, terms).generate_rust(options),
        Lang::Cxx => cxx::Codegen::compile(typeenv, termenv, terms).generate_cxx(options),
    }
}

struct BodyContext<'a, W> {
    out: &'a mut W,
    ruleset: &'a RuleSet,
    indent: String,
    is_ref: StableSet<BindingId>,
    is_bound: StableSet<BindingId>,
}

impl<'a, W: Write> BodyContext<'a, W> {
    fn new(out: &'a mut W, ruleset: &'a RuleSet) -> Self {
        Self {
            out,
            ruleset,
            indent: Default::default(),
            is_ref: Default::default(),
            is_bound: Default::default(),
        }
    }

    fn enter_scope(&mut self) -> StableSet<BindingId> {
        let new = self.is_bound.clone();
        std::mem::replace(&mut self.is_bound, new)
    }

    fn begin_block(&mut self) -> std::fmt::Result {
        self.indent.push_str("    ");
        writeln!(self.out, " {{")
    }

    fn end_block(&mut self, scope: StableSet<BindingId>) -> std::fmt::Result {
        self.is_bound = scope;
        self.end_block_without_newline()?;
        writeln!(self.out)
    }

    fn end_block_without_newline(&mut self) -> std::fmt::Result {
        self.indent.truncate(self.indent.len() - 4);
        write!(self.out, "{}}}", &self.indent)
    }

    fn set_ref(&mut self, binding: BindingId, is_ref: bool) {
        if is_ref {
            self.is_ref.insert(binding);
        } else {
            debug_assert!(!self.is_ref.contains(&binding));
        }
    }
}

mod rust;
mod cxx;
