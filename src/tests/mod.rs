use insta::{assert_snapshot, glob};
use oxc::{
  allocator::Allocator, codegen::CodegenOptions, minifier::MinifierOptions, span::SourceType,
};
use std::fs;

use crate::{TreeShakeConfig, TreeShakeOptions};

fn tree_shake(input: String) -> String {
  let do_minify = input.contains("@minify");
  let result = crate::tree_shake(TreeShakeOptions {
    config: TreeShakeConfig::default(),
    allocator: &Allocator::default(),
    source_type: SourceType::default(),
    source_text: input,
    tree_shake: true,
    minify: do_minify.then(|| MinifierOptions::default()),
    code_gen: CodegenOptions::default(),
    eval_mode: false,
    logging: true,
  });
  result.codegen_return.source_text
}

#[test]
fn test() {
  glob!("fixtures/**/*.js", |path| {
    let input = fs::read_to_string(path).unwrap();
    let mut filename: &str = "";
    if let Some(basename) = path.file_name() {
      filename = basename.to_str().unwrap();
    };

    assert_snapshot!(format!("test@{filename}"), tree_shake(input));
  });
}
