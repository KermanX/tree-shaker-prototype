mod arguments;
mod array;
mod builtin_fn;
mod collected;
mod collector;
mod computed;
mod consumed_object;
mod dep;
mod entity;
mod function;
mod label;
mod literal;
mod object;
mod operations;
mod promise;
mod symbol;
mod typeof_result;
mod union;
mod unknown;
mod utils;

pub use arguments::ArgumentsEntity;
pub use array::ArrayEntity;
pub use builtin_fn::{ImplementedBuiltinFnEntity, PureBuiltinFnEntity};
pub use collected::CollectedEntity;
pub use collector::LiteralCollector;
pub use computed::ComputedEntity;
pub use dep::EntityDepNode;
pub use entity::{Entity, EntityTrait};
pub use function::{FunctionEntity, FunctionEntitySource};
pub use label::LabelEntity;
pub use literal::LiteralEntity;
pub use object::{ObjectEntity, ObjectProperty, ObjectPropertyValue};
pub use operations::EntityOpHost;
pub use promise::PromiseEntity;
pub use typeof_result::TypeofResult;
pub use union::UnionEntity;
pub use unknown::UnknownEntity;

pub type EntryEntity<'a> = ComputedEntity<'a>;
pub type ForwardedEntity<'a> = ComputedEntity<'a>;
