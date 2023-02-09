use crate::sema::{ExternalSig, ReturnKind, Sym, Term, TermEnv, TermId, Type, TypeEnv, TypeId};
use crate::serialize::{Block, ControlFlow, EvalStep, MatchArm};
use crate::trie_again::{Binding, BindingId, Constraint, RuleSet};
use std::fmt::Write;

use super::*;

#[derive(Clone, Debug)]
pub(super) struct Codegen<'a> {
    typeenv: &'a TypeEnv,
    termenv: &'a TermEnv,
    terms: &'a [(TermId, RuleSet)],
}

impl<'a> Codegen<'a> {
    pub(super) fn compile(
        typeenv: &'a TypeEnv,
        termenv: &'a TermEnv,
        terms: &'a [(TermId, RuleSet)],
    ) -> Codegen<'a> {
        Codegen {
            typeenv,
            termenv,
            terms,
        }
    }

    pub(super) fn generate_cxx(&self, options: &CodegenOptions) -> String {
        let mut code = String::new();
        code
    }
}
