use std::marker::PhantomData;

use crate::mj_body::MjBodyChild;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-button";

pub struct MjButtonTag;

impl StaticTag for MjButtonTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjButton =
    Component<PhantomData<MjButtonTag>, crate::prelude::AttributeMap, Vec<MjBodyChild>>;
