use crate::ast::AstType2;
use crate::entity::entity::Entity;
use crate::{transformer::Transformer, Analyzer};
use oxc::ast::ast::{FormalParameter, FormalParameters};

const AST_TYPE: AstType2 = AstType2::FormalParameters;

#[derive(Debug, Default, Clone)]
pub struct Data {}

impl<'a> Analyzer<'a> {
  pub fn exec_formal_parameters(&mut self, node: &'a FormalParameters<'a>, args: Entity<'a>) {
    let resolved = args.get_to_array(node.items.len());

    for (param, arg) in node.items.iter().zip(resolved.0) {
      self.exec_binding_pattern(&param.pattern, (false, arg), false);
    }

    if let Some(rest) = &node.rest {
      self.exec_binding_rest_element_from_arr(rest, resolved.1, false);
    }
  }
}

impl<'a> Transformer<'a> {
  pub fn transform_formal_parameters(
    &mut self,
    node: FormalParameters<'a>,
  ) -> FormalParameters<'a> {
    let data = self.get_data::<Data>(AST_TYPE, &node);
    let FormalParameters { span, items, rest, kind, .. } = node;

    let mut transformed_items = self.ast_builder.vec();

    for param in items {
      let FormalParameter { span, decorators, pattern, .. } = param;
      let pattern = self.transform_binding_pattern(pattern);
      transformed_items.push(self.ast_builder.formal_parameter(
        span,
        decorators,
        pattern.unwrap_or_else(|| self.build_unused_binding_pattern(span)),
        None,
        false,
        false,
      ));
    }

    let transformed_rest = match rest {
      Some(rest) => self.transform_binding_rest_element(rest.unbox()),
      None => None,
    };

    self.ast_builder.formal_parameters(span, kind, transformed_items, transformed_rest)
  }
}
