use crate::{analyzer::Analyzer, ast::AstType2, data::get_node_ptr, transformer::Transformer};
use core::hash::Hash;
use oxc::{ast::AstKind, span::GetSpan};
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityDepNode {
  Environment,
  AstKind((usize, usize)),
  Ptr(AstType2, usize),
}

impl Debug for EntityDepNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      EntityDepNode::Environment => {
        f.write_str("Environment")?;
      }
      EntityDepNode::AstKind(node) => {
        let node = unsafe { std::mem::transmute::<_, AstKind<'static>>(*node) };
        node.span().fmt(f)?;
      }
      EntityDepNode::Ptr(t, s) => {
        (*t).fmt(f)?;
        s.fmt(f)?;
      }
    }
    Ok(())
  }
}

impl<'a> From<AstKind<'a>> for EntityDepNode {
  fn from(node: AstKind<'a>) -> Self {
    EntityDepNode::AstKind(unsafe { std::mem::transmute(node) })
  }
}

impl<T: GetSpan> From<(AstType2, &T)> for EntityDepNode {
  fn from((ty, node): (AstType2, &T)) -> Self {
    EntityDepNode::Ptr(ty, get_node_ptr(node))
  }
}

impl<'a> Analyzer<'a> {
  pub fn refer_dep(&mut self, dep: impl Into<EntityDepNode>) {
    self.referred_nodes.insert(dep.into());
  }
}

impl<'a> Transformer<'a> {
  pub fn refer_dep(&self, dep: impl Into<EntityDepNode>) {
    self.referred_nodes.borrow_mut().insert(dep.into());
  }

  pub fn is_referred(&self, dep: impl Into<EntityDepNode>) -> bool {
    self.referred_nodes.borrow().contains(&dep.into())
  }
}
