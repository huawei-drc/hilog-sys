//! hilog-sys
//!
//! Rust bindings for the `HiLog` logging framework of OpenHarmony.
//! This crate should only be used on OpenHarmony (`target_env = "ohos"`).
//! More information on hilog in native applications is available in the [hilog native guidelines].
//!
//! ## Safety
//!
//! When using `OH_LOG_Print` from Rust you **must** ensure that the `fmt` parameter either
//! - Does not contain any `printf` style format specifiers (like `%s`, `%d`) OR
//! - `fmt` is `"${public}s\0"` and the actual string is passed as the following parameter.
//!
//! [hilog native guidelines]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/dfx/hilog-guidelines.md

#[allow(non_snake_case)]
mod ffi;

pub use ffi::*;
