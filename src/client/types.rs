use crate::tl::TonNotification;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub struct TonError {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for TonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TonError code: {}, message: {}", self.code, self.message)
    }
}

impl Error for TonError {}

pub type TonNotificationReceiver = broadcast::Receiver<Arc<TonNotification>>;
