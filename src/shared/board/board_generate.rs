use crate::shared::coord::point::Point;

pub trait BoardGenerate {
    type Value;
    fn generate(f: impl Fn(&Point) -> Self::Value) -> Self;
}
