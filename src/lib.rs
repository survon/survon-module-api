use std::os::raw::c_char;

/// The common interface that every module must implement.
pub trait Module {
    /// Returns the unique namespace for the module instance.
    fn namespace(&self) -> &str;
    /// Returns a text representation to display (e.g. widget content).
    fn render(&self) -> String;
}

/// The function signature that every dynamic module must export.
/// When called, it will create a new module instance (Boxed trait object)
/// based on a provided name (as a C string).
#[allow(improper_ctypes_definitions)]
pub type ModuleCreateFunc = unsafe extern "C" fn(name: *const c_char) -> Box<dyn Module>;
