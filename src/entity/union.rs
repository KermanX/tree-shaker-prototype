use super::{
  entity::{Entity, EntityTrait},
  literal::LiteralEntity,
  typeof_result::TypeofResult,
  unknown::UnknownEntity,
  utils::collect_effect_and_value,
};
use crate::analyzer::Analyzer;
use rustc_hash::FxHashSet;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct UnionEntity<'a>(pub Vec<Entity<'a>>);

impl<'a> EntityTrait<'a> for UnionEntity<'a> {
  fn consume_self(&self, analyzer: &mut Analyzer<'a>) {
    for entity in &self.0 {
      entity.consume_self(analyzer);
    }
  }

  fn consume_as_unknown(&self, analyzer: &mut Analyzer<'a>) {
    for entity in &self.0 {
      entity.consume_as_unknown(analyzer);
    }
  }

  fn get_property(&self, analyzer: &mut Analyzer<'a>, key: &Entity<'a>) -> (bool, Entity<'a>) {
    let mut values = Vec::new();
    for entity in &self.0 {
      values.push(entity.get_property(analyzer, key));
    }
    collect_effect_and_value(values)
  }

  fn set_property(&self, analyzer: &mut Analyzer<'a>, key: &Entity<'a>, value: Entity<'a>) -> bool {
    let mut has_effect = false;
    for entity in &self.0 {
      has_effect |= entity.set_property(analyzer, key, value.clone());
    }
    has_effect
  }

  fn enumerate_properties(
    &self,
    _analyzer: &mut Analyzer<'a>,
  ) -> (bool, Vec<(bool, Entity<'a>, Entity<'a>)>) {
    // FIXME:
    UnknownEntity::new_unknown_to_entries_result(self.0.clone())
  }

  fn call(
    &self,
    analyzer: &mut Analyzer<'a>,
    this: &Entity<'a>,
    args: &Entity<'a>,
  ) -> (bool, Entity<'a>) {
    let mut values = Vec::new();
    for entity in &self.0 {
      values.push(entity.call(analyzer, this, args));
    }
    collect_effect_and_value(values)
  }

  fn get_typeof(&self) -> Entity<'a> {
    let mut result = Vec::new();
    // TODO: collect literals
    for entity in &self.0 {
      result.push(entity.get_typeof());
    }
    Rc::new(UnionEntity(result))
  }

  fn get_to_string(&self) -> Entity<'a> {
    let mut result = Vec::new();
    // TODO: dedupe
    for entity in &self.0 {
      result.push(entity.get_to_string());
    }
    Rc::new(UnionEntity(result))
  }

  fn get_to_property_key(&self) -> Entity<'a> {
    let mut result = Vec::new();
    // TODO: dedupe
    for entity in &self.0 {
      result.push(entity.get_to_property_key());
    }
    Rc::new(UnionEntity(result))
  }

  fn get_to_array(&self, length: usize) -> (Vec<Entity<'a>>, Entity<'a>) {
    // FIXME: May have the same result
    let mut elements = Vec::new();
    for _ in 0..length {
      elements.push(Vec::new());
    }
    let mut rest = Vec::new();
    for entity in &self.0 {
      let result = entity.get_to_array(length);
      for (i, element) in elements.iter_mut().enumerate() {
        element.push(result.0[i].clone());
      }
      rest.push(result.1);
    }
    (elements.into_iter().map(UnionEntity::new).collect(), UnionEntity::new(rest))
  }

  fn get_to_literals(&self) -> Option<FxHashSet<LiteralEntity<'a>>> {
    let mut result = self.0.first().unwrap().get_to_literals()?;
    for entity in &self.0[1..] {
      result.extend(entity.get_to_literals()?);
    }
    Some(result)
  }

  fn test_typeof(&self) -> TypeofResult {
    let mut result = TypeofResult::_None;
    for entity in &self.0 {
      result |= entity.test_typeof();
    }
    result
  }

  fn test_truthy(&self) -> Option<bool> {
    let result = self.0.first().unwrap().test_truthy()?;
    for entity in &self.0[1..] {
      if entity.test_truthy()? != result {
        return None;
      }
    }
    Some(result)
  }

  fn test_nullish(&self) -> Option<bool> {
    let result = self.0.first().unwrap().test_nullish()?;
    for entity in &self.0[1..] {
      if entity.test_nullish()? != result {
        return None;
      }
    }
    Some(result)
  }
}

impl<'a> UnionEntity<'a> {
  pub(crate) fn new(entities: Vec<Entity<'a>>) -> Entity<'a> {
    debug_assert!(!entities.is_empty());
    if entities.len() == 1 {
      entities.first().unwrap().clone()
    } else {
      Rc::new(UnionEntity(entities))
    }
  }
}
