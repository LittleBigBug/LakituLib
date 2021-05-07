pub trait LakituPlugin {
    fn plugin_enable(&self) -> Result<(), Err>;
    fn plugin_disable(&self) -> Result<(), Err>;
}

pub trait PluginRegistrar {
    fn register_func(&mut self, name: &str, function: Box<dyn Function>);
}

pub struct LakituPluginDeclaration {
    pub rustc_version: &'static str,
    pub lakitu_lib_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
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