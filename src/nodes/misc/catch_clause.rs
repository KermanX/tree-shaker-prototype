use crate::{analyzer::Analyzer, entity::entity::Entity, transformer::Transformer};
use oxc::{
  ast::ast::{CatchClause, CatchParameter},
  span::GetSpan,
};

impl<'a> Analyzer<'a> {
  pub fn exec_catch_clause(&mut self, node: &'a CatchClause<'a>, value: Entity<'a>) {
    let cf_scope_id = self.push_cf_scope(Some(false), false);
    self.push_variable_scope(cf_scope_id);

    if let Some(param) = &node.param {
      self.exec_binding_pattern(&param.pattern, (false, value), false);
    }

    self.exec_block_statement(&node.body);

    self.pop_variable_scope();
    self.pop_cf_scope();
  }
}

impl<'a> Transformer<'a> {
  pub fn transform_catch_clause(&self, node: CatchClause<'a>) -> CatchClause<'a> {
    let CatchClause { span, param, body, .. } = node;

    let param = param.and_then(|param| {
      let CatchParameter { span, pattern, .. } = param;
      self
        .transform_binding_pattern(pattern)
        .map(|pattern| self.ast_builder.catch_parameter(span, pattern))
    });

    let body_span = body.span();
    let body = self.transform_block_statement(body.unbox());

    self.ast_builder.catch_clause(
      span,
      param,
      body.unwrap_or(self.ast_builder.block_statement(body_span, self.ast_builder.vec())),
    )
  }
}
