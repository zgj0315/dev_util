//! # dev_util
//! Some development tools
//!
//! # Usage Examples
//! ## log_init
//! Add this to your *Cargo.toml*:
//! ```toml
//! [dependencies]
//! log = "0.4"
//!
//! [dev-dependencies]
//! dev_util = "0.1"
//! ```
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use dev_util::log::log_init;
//!     #[test]
//!     fn it_works() {
//!         log::info!("log is't initialized, you can't see me");
//!         log_init();
//!         log::info!("log is initialized");
//!     }
//! }
//!
//! ```
//!

pub mod log;
