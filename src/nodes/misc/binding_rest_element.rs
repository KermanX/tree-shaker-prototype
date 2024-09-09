use crate::ast::AstType2;
use crate::entity::entity::{Entity, EntityTrait};
use crate::{transformer::Transformer, Analyzer};
use oxc::ast::ast::{BindingRestElement, PropertyKind, VariableDeclarationKind};
use oxc::span::GetSpan;

const AST_TYPE: AstType2 = AstType2::BindingRestElement;

#[derive(Debug, Default)]
struct Data {
  has_effect: bool,
}

impl<'a> Analyzer<'a> {
  pub fn exec_binding_rest_element_from_obj(
    &mut self,
    node: &'a BindingRestElement<'a>,
    init: Entity<'a>,
    enumerated_keys: Vec<Entity<'a>>,
    exporting: bool,
    kind: VariableDeclarationKind,
  ) {
    let (has_effect, properties) = init.enumerate_properties(self);

    let object = self.new_empty_object();
    for (definite, key, value) in properties {
      object.init_property(PropertyKind::Init, key, value, definite);
    }

    for key in enumerated_keys {
      object.delete_property(self, &key);
    }

    self.exec_binding_pattern(&node.argument, (has_effect, init), exporting, kind);

    let data = self.load_data::<Data>(AST_TYPE, node);
    data.has_effect |= has_effect;
  }

  pub fn exec_binding_rest_element_from_arr(
    &mut self,
    node: &'a BindingRestElement<'a>,
    init: Entity<'a>,
    exporting: bool,
    kind: VariableDeclarationKind,
  ) {
    todo!()
  }
}

impl<'a> Transformer<'a> {
  pub fn transform_binding_rest_element(
    &self,
    node: &'a BindingRestElement<'a>,
  ) -> Option<BindingRestElement<'a>> {
    let data = self.get_data::<Data>(AST_TYPE, node);

    let BindingRestElement { span, argument, .. } = node;
    let argument_span = argument.span();

    let argument = self.transform_binding_pattern(argument);

    if let Some(argument) = argument {
      Some(self.ast_builder.binding_rest_element(*span, argument))
    } else if data.has_effect {
      Some(
        self
          .ast_builder
          .binding_rest_element(*span, self.build_unused_binding_pattern(argument_span)),
      )
    } else {
      None
    }
  }
}
