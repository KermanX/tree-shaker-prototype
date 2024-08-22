use crate::ast_type::AstType2;
use crate::{transformer::Transformer, Analyzer};
use oxc::{
  ast::{
    ast::{ExpressionStatement, Statement},
    match_declaration, match_module_declaration,
  },
  span::GetSpan,
};

const AST_TYPE: AstType2 = AstType2::Statement;

#[derive(Debug, Default, Clone)]
pub struct Data {}

impl<'a> Analyzer<'a> {
  pub(crate) fn exec_statement(&mut self, node: &'a Statement) -> bool {
    match node {
      match_declaration!(Statement) => {
        let node = node.to_declaration();
        self.exec_declaration(node)
      }
      match_module_declaration!(Statement) => {
        let node = node.to_module_declaration();
        self.exec_module_declaration(node);
        false
      }
      Statement::ExpressionStatement(node) => self.exec_expression(&node.expression).0,
      Statement::BlockStatement(node) => self.exec_block_statement(node),
      Statement::IfStatement(node) => self.exec_if_statement(node),
      _ => todo!(),
    }
  }
}

impl<'a> Transformer<'a> {
  pub(crate) fn transform_statement(&self, node: Statement<'a>) -> Option<Statement<'a>> {
    let span = node.span();
    match node {
      match_declaration!(Statement) => self
        .transform_declaration(node.try_into().unwrap())
        .map(|decl| self.ast_builder.statement_declaration(decl)),
      match_module_declaration!(Statement) => {
        Some(self.ast_builder.statement_module_declaration(
          self.transform_module_declaration(node.try_into().unwrap()),
        ))
      }
      Statement::ExpressionStatement(node) => {
        let ExpressionStatement { expression, .. } = node.unbox();
        self
          .transform_expression(expression, false)
          .map(|expr| self.ast_builder.statement_expression(span, expr))
      }
      Statement::BlockStatement(node) => self.transform_block_statement(node.unbox()),
      Statement::IfStatement(node) => self.transform_if_statement(node.unbox()),
      _ => todo!(),
    }
  }
}
