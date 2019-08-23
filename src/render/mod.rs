use std::rc::Rc;
use std::fmt;
use std::error::Error;
use ash::{self, vk, vk_make_version};
use ash::{Entry, version::EntryV1_0, Instance, version::InstanceV1_0};
use ash::extensions::khr::Surface;
use crate::c_str;

const REQUIRED_API_VERSION: u32 = vk_make_version!(1, 0, 0);

#[derive(Debug)]
pub enum InitializationError {
    LibraryLoadError(String),
    LoadError(Vec<&'static str>),
    VkError(vk::Result),
}

impl Error for InitializationError {}

impl fmt::Display for InitializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LibraryLoadError(msg) => write!(f, "{}", msg),
            Self::LoadError(err) => write!(f, "{}", err.join("; ")),
            Self::VkError(err) => write!(f, "{}", err),
        }
    }
}
0
pub struct GraphicsDispatch {
    pub entry: Entry,
    pub instance: Instance,
    pub surface: Surface
}

impl GraphicsDispatch {
    pub fn new() -> Result<Self, InitializationError> {
        let entry = ash::Entry::new()
            .map_err(|e| match e {
                ash::LoadingError::LibraryLoadError(msg) => InitializationError::LibraryLoadError(msg)
            })?;

        let app_info = vk::ApplicationInfo {
            p_application_name: c_str!("Biserialis").as_ptr(),
            api_version: REQUIRED_API_VERSION,
            ..Default::default()
        };

        let extensions = [Surface::name().as_ptr()];

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            ..Default::default()
        };

        let instance = unsafe {
            entry.create_instance(&create_info, None)
                .map_err(|e| match e {
                    ash::InstanceError::LoadError(err) => InitializationError::LoadError(err),
                    ash::InstanceError::VkError(err) => InitializationError::VkError(err)
                })?
        };

        let surface = Surface::new(&entry, &instance);

        Ok(GraphicsDispatch {
            entry,
            instance,
            surface
        })
    }
}
