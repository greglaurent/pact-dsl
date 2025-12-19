pub trait Lang {}

pub struct Rust;
impl Lang for Rust {}

pub struct Zig;
impl Lang for Zig {}

pub trait Generator<L: Lang> {}

pub trait Translate: Sized {}
