use std::fmt::Display;

pub trait Node: Display {
    type Element;
    fn get(&self) -> Self::Element;
}
