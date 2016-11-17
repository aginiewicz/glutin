pub mod android;
pub mod caca;
pub mod dlopen;
pub mod egl;
pub mod emscripten;
pub mod glx;
pub mod osmesa;
pub mod wgl;
pub mod ios;

use std::os::raw::c_void;

pub type SomeContext = *const c_void;

pub type SomeDisplay = *const c_void;

pub enum SharedContextForCL {
  Egl{context: SomeContext, display: SomeDisplay},
  Glx{context: SomeContext, display: SomeDisplay},
  Wgl{context: SomeContext, handle: SomeDisplay},
  Cgl{sharegroup: SomeDisplay},
  NotAvailable
}
