use super::{
  dep::EntityDep,
  entity::{Entity, EntityTrait},
  forwarded::ForwardedEntity,
  literal::LiteralEntity,
  typeof_result::TypeofResult,
  unknown::{UnknownEntity, UnknownEntityKind},
};
use crate::{
  analyzer::Analyzer,
  entity::{consumed_object, dep::ENVIRONMENT_DEP},
  scope::variable_scope::{VariableScope, VariableScopes},
  use_consumed_flag,
};
use oxc::ast::{
  ast::{ArrowFunctionExpression, Function},
  AstKind,
};
use std::{
  cell::{Cell, RefCell},
  rc::Rc,
};

#[derive(Debug, Clone, Copy)]
pub enum FunctionEntitySource<'a> {
  Function(&'a Function<'a>),
  ArrowFunctionExpression(&'a ArrowFunctionExpression<'a>),
}

#[derive(Debug, Clone)]
pub struct FunctionEntity<'a> {
  consumed: Cell<bool>,
  pub source: FunctionEntitySource<'a>,
  pub variable_scopes: Rc<VariableScopes<'a>>,
}

impl<'a> EntityTrait<'a> for FunctionEntity<'a> {
  fn consume_self(&self, analyzer: &mut Analyzer<'a>) {
    analyzer.refer_dep(self.dep());
  }

  fn consume_as_unknown(&self, analyzer: &mut Analyzer<'a>) {
    use_consumed_flag!(self);

    self.consume_self(analyzer);

    analyzer.exec_exhaustively(|analyzer| {
      analyzer.push_cf_scope_normal(None);
      let ret_val = self.call(
        analyzer,
        ENVIRONMENT_DEP,
        &UnknownEntity::new_unknown(),
        &UnknownEntity::new_unknown(),
      );
      analyzer.pop_cf_scope();

      ret_val.consume_as_unknown(analyzer);
    });
  }

  fn get_property(
    &self,
    _rc: &Entity<'a>,
    analyzer: &mut Analyzer<'a>,
    dep: EntityDep,
    key: &Entity<'a>,
  ) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_property(analyzer, dep, key);
    }
    analyzer.builtins.prototypes.function.get_property(key, dep)
  }

  fn set_property(
    &self,
    _rc: &Entity<'a>,
    analyzer: &mut Analyzer<'a>,
    dep: EntityDep,
    key: &Entity<'a>,
    value: Entity<'a>,
  ) {
    self.consume_as_unknown(analyzer);
    consumed_object::set_property(analyzer, dep, key, value)
  }

  fn delete_property(&self, analyzer: &mut Analyzer<'a>, key: &Entity<'a>) -> bool {
    self.consume_as_unknown(analyzer);
    consumed_object::delete_property(analyzer, key)
  }

  fn enumerate_properties(
    &self,
    _rc: &Entity<'a>,
    analyzer: &mut Analyzer<'a>,
    dep: EntityDep,
  ) -> Vec<(bool, Entity<'a>, Entity<'a>)> {
    self.consume_as_unknown(analyzer);
    consumed_object::enumerate_properties(analyzer, dep)
  }

  fn call(
    &self,
    analyzer: &mut Analyzer<'a>,
    dep: EntityDep,
    this: &Entity<'a>,
    args: &Entity<'a>,
  ) -> Entity<'a> {
    let variable_scopes = self.variable_scopes.clone();
    let ret_val = match self.source {
      FunctionEntitySource::Function(node) => {
        analyzer.call_function(dep, node, variable_scopes, this.clone(), args.clone())
      }
      FunctionEntitySource::ArrowFunctionExpression(node) => {
        analyzer.call_arrow_function_expression(dep, node, variable_scopes, args.clone())
      }
    };
    ForwardedEntity::new(ret_val, self.dep())
  }

  fn r#await(&self, rc: &Entity<'a>, analyzer: &mut Analyzer<'a>) -> (bool, Entity<'a>) {
    if self.consumed.get() {
      return consumed_object::r#await(analyzer);
    }
    (false, rc.clone())
  }

  fn iterate(&self, rc: &Entity<'a>, analyzer: &mut Analyzer<'a>) -> (bool, Option<Entity<'a>>) {
    if self.consumed.get() {
      return consumed_object::iterate(analyzer);
    }
    (true, Some(UnknownEntity::new_unknown_with_deps(vec![rc.clone()])))
  }

  fn get_typeof(&self) -> Entity<'a> {
    LiteralEntity::new_string("function")
  }

  fn get_to_string(&self, rc: &Entity<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_to_string();
    }
    UnknownEntity::new_with_deps(UnknownEntityKind::String, vec![rc.clone()])
  }

  fn get_to_property_key(&self, rc: &Entity<'a>) -> Entity<'a> {
    self.get_to_string(rc)
  }

  fn get_to_array(&self, rc: &Entity<'a>, length: usize) -> (Vec<Entity<'a>>, Entity<'a>) {
    if self.consumed.get() {
      return consumed_object::get_to_array(length);
    }
    UnknownEntity::new_unknown_to_array_result(length, vec![rc.clone()])
  }

  fn test_typeof(&self) -> TypeofResult {
    TypeofResult::Function
  }

  fn test_truthy(&self) -> Option<bool> {
    Some(true)
  }

  fn test_nullish(&self) -> Option<bool> {
    Some(false)
  }
}

impl<'a> FunctionEntity<'a> {
  pub fn new(
    source: FunctionEntitySource<'a>,
    variable_scopes: Vec<Rc<RefCell<VariableScope<'a>>>>,
  ) -> Entity<'a> {
    Entity::new(Self {
      consumed: Cell::new(false),
      source,
      variable_scopes: Rc::new(variable_scopes),
    })
  }

  pub fn dep(&self) -> EntityDep {
    EntityDep::from(match self.source {
      FunctionEntitySource::Function(node) => AstKind::Function(node),
      FunctionEntitySource::ArrowFunctionExpression(node) => AstKind::ArrowFunctionExpression(node),
    })
  }
}
