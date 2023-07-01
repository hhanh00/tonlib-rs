use base64_serde::base64_serde_type;

mod function;
mod notification;
mod result;
mod serial;
pub mod stack;
pub mod types;

pub use function::TonFunction;
pub use notification::TonNotification;
pub use result::TonResult;

base64_serde_type!(Base64Standard, base64::STANDARD);
