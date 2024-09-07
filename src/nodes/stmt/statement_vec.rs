use crate::{analyzer::Analyzer, data::StatementVecData, transformer::Transformer};
use oxc::{allocator::Vec, ast::ast::Statement};

impl<'a> Analyzer<'a> {
  pub fn exec_statement_vec(
    &mut self,
    data: &mut StatementVecData,
    exited: Option<bool>,
    statements: &'a Vec<'a, Statement<'a>>,
  ) {
    let cf_scope_id = self.push_cf_scope(exited, false);
    self.push_variable_scope(cf_scope_id);

    let mut last_stmt = None;
    for (index, statement) in statements.iter().enumerate() {
      if self.cf_scope().must_exited() {
        break;
      }
      self.exec_statement(statement);
      last_stmt = Some(index);
    }
    if let Some(last_stmt) = last_stmt {
      data.last_stmt = match data.last_stmt {
        Some(old_last_stmt) => Some(old_last_stmt.max(last_stmt)),
        None => Some(last_stmt),
      };
    }

    self.pop_variable_scope();
    self.pop_cf_scope();
  }
}

impl<'a> Transformer<'a> {
  pub fn transform_statement_vec(
    &self,
    data: &StatementVecData,
    statements: &'a Vec<'a, Statement<'a>>,
  ) -> Vec<'a, Statement<'a>> {
    let mut result = self.ast_builder.vec();

    if data.last_stmt.is_none() {
      return result;
    }

    for (index, statement) in statements.iter().enumerate() {
      if let Some(statement) = self.transform_statement(statement) {
        result.push(statement);
      }

      if data.last_stmt == Some(index) {
        break;
      }
    }

    result
  }
}
