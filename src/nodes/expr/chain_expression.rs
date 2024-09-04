use crate::{analyzer::Analyzer, entity::entity::Entity, transformer::Transformer};
use oxc::ast::{
  ast::{ChainElement, ChainExpression, Expression},
  match_member_expression,
};

impl<'a> Analyzer<'a> {
  pub fn exec_chain_expression(&mut self, node: &'a ChainExpression<'a>) -> Entity<'a> {
    match &node.expression {
      ChainElement::CallExpression(node) => self.exec_call_expression(node),
      node => self.exec_member_expression_read(node.to_member_expression()),
    }
  }
}

impl<'a> Transformer<'a> {
  pub fn transform_chain_expression(
    &self,
    node: ChainExpression<'a>,
    need_val: bool,
  ) -> Option<Expression<'a>> {
    let ChainExpression { span, expression } = node;

    let expression = match expression {
      ChainElement::CallExpression(node) => self.transform_call_expression(node.unbox(), need_val),
      node => self.transform_member_expression_read(node.try_into().unwrap(), need_val),
    };

    // FIXME: is this correct?
    expression.map(|expression| match expression {
      Expression::CallExpression(node) => self
        .ast_builder
        .expression_chain(span, self.ast_builder.chain_element_from_call_expression(node)),
      match_member_expression!(Expression) => self.ast_builder.expression_chain(
        span,
        self.ast_builder.chain_element_member_expression(expression.try_into().unwrap()),
      ),
      _ => expression,
    })
  }
}
