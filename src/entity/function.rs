use super::{
  dep::{EntityDep, EntityDepNode},
  entity::{Entity, EntityTrait},
  forwarded::ForwardedEntity,
  unknown::UnknownEntity,
};
use crate::analyzer::Analyzer;
use std::{rc::Rc, vec};

#[derive(Debug)]
pub(crate) struct FunctionEntity<'a> {
  pub(crate) source: EntityDep<'a>,
}

impl<'a> EntityTrait<'a> for FunctionEntity<'a> {
  fn consume_self(&self, analyzer: &mut Analyzer<'a>) {
    analyzer.refer_dep(&self.source);
  }

  fn consume_as_unknown(&self, analyzer: &mut Analyzer<'a>) {
    self.consume_self(analyzer);
    self.call(analyzer, &UnknownEntity::new_unknown(), &UnknownEntity::new_unknown());
  }

  fn call(
    &self,
    analyzer: &mut Analyzer<'a>,
    this: &Entity<'a>,
    args: &Entity<'a>,
  ) -> (bool, Entity<'a>) {
    match &self.source.node {
      EntityDepNode::Function(node) => {
        let (has_effect, ret_val) = analyzer.call_function(node, this.clone(), args.clone());
        if has_effect {
          self.consume_self(analyzer);
        }
        (has_effect, ForwardedEntity::new(ret_val, self.source.clone()))
      }
      EntityDepNode::ArrowFunctionExpression(node) => todo!(),
      _ => unreachable!(),
    }
  }

  fn get_property(&self, _key: &Entity<'a>) -> Entity<'a> {
    todo!("built-ins")
  }

  fn test_truthy(&self) -> Option<bool> {
    Some(true)
  }

  fn test_nullish(&self) -> Option<bool> {
    Some(false)
  }
}

impl<'a> FunctionEntity<'a> {
  pub(crate) fn new(source: EntityDep<'a>) -> Entity<'a> {
    Rc::new(Self { source })
  }
}
