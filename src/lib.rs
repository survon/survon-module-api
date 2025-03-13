use std::os::raw::c_char;

/// The common API that every module must implement.
/// It requires `Send` so that module instances can be safely transferred between threads.
pub trait Module: Send {
    /// Returns the unique namespace for the module instance.
    fn namespace(&self) -> &str;
    /// Returns a text representation to display.
    fn render(&self) -> String;
}

/// The function signature that every dynamic module must export.
/// This function creates a new module instance from a C string name.
pub type ModuleCreateFunc = unsafe extern "C" fn(name: *const c_char) -> Box<dyn Module>;
