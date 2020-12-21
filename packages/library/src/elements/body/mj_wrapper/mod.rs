mod parser;
mod renderer;

use super::BodyElement;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-wrapper";

const DEFAULT_BACKGROUND_POSITION: &str = "top center";

#[derive(Clone, Debug)]
pub struct MJWrapper {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}