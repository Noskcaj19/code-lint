#![feature(macro_vis_matcher)]
#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]

extern crate syntax;

#[macro_use]
extern crate rustc;
extern crate rustc_plugin;

use rustc::lint::{EarlyContext, EarlyLintPass, EarlyLintPassObject, LintArray, LintContext,
                  LintPass};
use rustc_plugin::Registry;
use syntax::ast;

declare_lint!(
    VALID_CODE_LINT,
    Warn,
    "Warn about the presence of valid code"
);

struct ValidCodePass;

impl LintPass for ValidCodePass {
    fn get_lints(&self) -> LintArray {
        lint_array!(VALID_CODE_LINT)
    }
}

impl EarlyLintPass for ValidCodePass {
    fn check_crate(&mut self, cx: &EarlyContext, cr: &ast::Crate) {
        let mut db = cx.struct_span_lint(VALID_CODE_LINT, cr.span, "valid code found");

        db.help("valid code is included");
        db.help("deleting all of the code or rewriting it from scratch might help");
        // cx.span_lint(VALID_CODE_LINT, cr.span, "valid code found");
        db.emit();
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_early_lint_pass(box ValidCodePass as EarlyLintPassObject);
}
