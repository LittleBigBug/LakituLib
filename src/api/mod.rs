use std::sync::RwLockWriteGuard;
use crate::command::LakituCommand;
use crate::platform::event::LakituEventManager;

pub trait LakituPlugin {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> &str;
    fn get_author(&self) -> &str;

    fn get_description(&self) -> &str;

    fn plugin_enable(&self) -> Result<(), Err>;
    fn plugin_disable(&self) -> Result<(), Err>;

    fn register_events(&self, event_manager: RwLockWriteGuard<Box<dyn LakituEventManager>>);
}

pub trait LakituPluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn LakituPlugin>);
}

pub struct LakituPluginDeclaration {
    pub rustc_version: &'static str,
    pub lakitu_lib_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn LakituPluginRegistrar),
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static lakitu_plugin_declaration: $crate::LakituPluginDeclaration = $crate::LakituPluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            lakitu_lib_version: $crate::LAKITU_LIB_VERSION,
            register: $register,
        }
    };
}