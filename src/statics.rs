// This file contains statics defined using lazy_static crate.

use crate::config::{Config};

lazy_static::lazy_static! {
	pub static ref CONFIG: crate::config::Config = Config::load();
}
