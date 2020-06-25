
use std::rc::Rc;
use crate::TwitchClientInner;

mod get;
pub use get::*;

pub struct ClipsGroup {
    client: Rc<TwitchClientInner>
}

impl ClipsGroup {
    pub(crate) fn new(client: Rc<TwitchClientInner>) -> Self {
        Self {
            client
        }
    }
}
