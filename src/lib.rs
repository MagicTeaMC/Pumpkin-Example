// https://docs.pumpkinmc.org/plugin-dev/plugin-template/basic-logic
use pumpkin_api_macros::plugin_impl;

#[plugin_impl]
pub struct Example {}

impl Example {
    pub fn new() -> Self {
        Example {}
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}