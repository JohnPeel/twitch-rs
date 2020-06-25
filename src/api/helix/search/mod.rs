
use std::rc::Rc;
use crate::TwitchClientInner;

mod category;
pub use category::*;

mod channel;
pub use channel::*;

pub struct SearchGroup {
    client: Rc<TwitchClientInner>
}

impl SearchGroup {
    pub(crate) fn new(client: Rc<TwitchClientInner>) -> Self {
        Self {
            client
        }
    }
}