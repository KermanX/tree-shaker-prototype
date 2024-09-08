use oxc::semantic::ScopeId;

use crate::analyzer::Analyzer;

use super::{
  entity::Entity,
  literal::LiteralEntity,
  union::UnionEntity,
  unknown::{UnknownEntity, UnknownEntityKind},
};

pub fn collect_effect_and_value<'a>(values: Vec<(bool, Entity<'a>)>) -> (bool, Entity<'a>) {
  let mut has_effect = false;
  let mut result = Vec::new();
  for (effect, value) in values {
    has_effect |= effect;
    result.push(value);
  }
  (has_effect, UnionEntity::new(result))
}

pub fn boolean_from_test_result<'a>(
  result: Option<bool>,
  deps: impl FnOnce() -> Vec<Entity<'a>>,
) -> Entity<'a> {
  match result {
    Some(value) => LiteralEntity::new_boolean(value),
    None => UnknownEntity::new_with_deps(UnknownEntityKind::Boolean, deps()),
  }
}

pub fn is_assignment_indeterminate<'a>(scope_path: &Vec<ScopeId>, analyzer: &Analyzer<'a>) -> bool {
  let mut var_scope_id = analyzer.scope_context.variable_scopes.first().unwrap().id;
  for (i, scope) in analyzer.scope_context.variable_scopes.iter().enumerate() {
    let scope_id = scope.id;
    if scope_path.get(i).is_some_and(|id| *id == scope_id) {
      var_scope_id = scope_id;
    } else {
      break;
    }
  }
  let target = analyzer.get_variable_scope_by_id(var_scope_id).cf_scope_index;
  analyzer.is_relative_indeterminate(target)
}
