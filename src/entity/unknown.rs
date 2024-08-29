use super::{
  entity::{Entity, EntityTrait},
  literal::LiteralEntity,
  typeof_result::TypeofResult,
};
use crate::analyzer::Analyzer;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub(crate) enum UnknownEntityKind {
  // TODO: NumericString, NoneEmptyString, ...
  String,
  Number,
  BigInt,
  Boolean,
  Symbol,
  Array,
  Function,
  Object,
  Unknown,
}

#[derive(Debug, Clone)]
pub(crate) struct UnknownEntity<'a> {
  pub kind: UnknownEntityKind,
  pub deps: Vec<Entity<'a>>,
}

impl<'a> EntityTrait<'a> for UnknownEntity<'a> {
  fn consume_self(&self, analyzer: &mut Analyzer<'a>) {
    for dep in &self.deps {
      dep.consume_self(analyzer);
    }
  }

  fn consume_as_unknown(&self, analyzer: &mut Analyzer<'a>) {
    for dep in &self.deps {
      dep.consume_as_unknown(analyzer);
    }
  }

  fn get_typeof(&self) -> Entity<'a> {
    if let Some(str) = self.test_typeof().to_string() {
      LiteralEntity::new_string(str)
    } else {
      UnknownEntity::new_with_deps(UnknownEntityKind::String, self.deps.clone())
    }
  }

  fn get_to_string(&self) -> Entity<'a> {
    UnknownEntity::new_with_deps(UnknownEntityKind::String, self.deps.clone())
  }

  fn get_to_property_key(&self) -> Entity<'a> {
    UnknownEntity::new_with_deps(UnknownEntityKind::Unknown, self.deps.clone())
  }

  fn get_property(&self, key: &Entity<'a>) -> Entity<'a> {
    // TODO: Builtin properties
    let mut deps = self.deps.clone();
    deps.push(key.clone());
    Rc::new(Self { kind: UnknownEntityKind::Unknown, deps })
  }

  fn get_to_array(&self, length: usize) -> (Vec<Entity<'a>>, Entity<'a>) {
    UnknownEntity::new_unknown_to_array_result(length, self.deps.clone())
  }

  fn test_typeof(&self) -> TypeofResult {
    match &self.kind {
      UnknownEntityKind::String => TypeofResult::String,
      UnknownEntityKind::Number => TypeofResult::Number,
      UnknownEntityKind::BigInt => TypeofResult::BigInt,
      UnknownEntityKind::Boolean => TypeofResult::Boolean,
      UnknownEntityKind::Symbol => TypeofResult::Symbol,
      UnknownEntityKind::Array => TypeofResult::Object,
      UnknownEntityKind::Function => TypeofResult::Function,
      UnknownEntityKind::Object => TypeofResult::Object,
      UnknownEntityKind::Unknown => TypeofResult::_Unknown,
    }
  }

  fn test_truthy(&self) -> Option<bool> {
    match &self.kind {
      UnknownEntityKind::Symbol
      | UnknownEntityKind::Array
      | UnknownEntityKind::Function
      | UnknownEntityKind::Object => Some(true),
      _ => None,
    }
  }

  fn test_nullish(&self) -> Option<bool> {
    match &self.kind {
      UnknownEntityKind::Unknown => None,
      _ => Some(false),
    }
  }
}

impl<'a> UnknownEntity<'a> {
  pub fn new_with_deps(kind: UnknownEntityKind, deps: Vec<Entity<'a>>) -> Entity<'a> {
    Rc::new(Self { kind, deps })
  }

  pub fn new(kind: UnknownEntityKind) -> Entity<'a> {
    Self::new_with_deps(kind, Vec::new())
  }

  pub fn new_unknown() -> Entity<'a> {
    Self::new(UnknownEntityKind::Unknown)
  }

  pub fn new_unknown_with_deps(deps: Vec<Entity<'a>>) -> Entity<'a> {
    Self::new_with_deps(UnknownEntityKind::Unknown, deps)
  }

  pub fn new_unknown_to_array_result(
    length: usize,
    deps: Vec<Entity<'a>>,
  ) -> (Vec<Entity<'a>>, Entity<'a>) {
    let mut result = Vec::new();
    let unknown = UnknownEntity::new_unknown_with_deps(deps);
    for _ in 0..length {
      result.push(unknown.clone());
    }
    (result, unknown)
  }
}
