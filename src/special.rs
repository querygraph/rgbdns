//! Shared UDP/TCP transport for specialized authoritative responders.

use crate::Result;
use std::sync::Arc;

pub type Handler = crate::transport::Handler;

pub fn serve(address: &str, handler: Arc<Handler>) -> Result<()> {
    crate::transport::serve(address, handler)
}
