use crate::{
  analyzer::Analyzer,
  entity::{
    dep::{EntityDep, EntityDepNode},
    forwarded::ForwardedEntity,
    literal::LiteralEntity,
  },
  transformer::Transformer,
};
use oxc::ast::ast::{ReturnStatement, Statement};

impl<'a> Analyzer<'a> {
  pub(crate) fn exec_return_statement(&mut self, node: &'a ReturnStatement) {
    let indeterminate = self.indeterminate;
    let value = node
      .argument
      .as_ref()
      .map_or_else(|| LiteralEntity::new_undefined(), |expr| self.exec_expression(expr));
    let dep = self.new_entity_dep(EntityDepNode::ReturnStatement(node));
    let value = ForwardedEntity::new(value, dep);
    self.function_scope_mut().on_return(indeterminate, value);
  }
}

impl<'a> Transformer<'a> {
  pub(crate) fn transform_return_statement(
    &mut self,
    node: ReturnStatement<'a>,
  ) -> Option<Statement<'a>> {
    let need_val = self.is_referred(EntityDepNode::ReturnStatement(&node));

    let ReturnStatement { span, argument } = node;

    Some(
      self
        .ast_builder
        .statement_return(span, argument.and_then(|arg| self.transform_expression(arg, need_val))),
    )
  }
}
