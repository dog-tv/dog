#![allow(clippy::needless_range_loop)]
#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]

pub mod examples;

#[doc(inline)]
pub use dog_tv_renderer as renderer;
#[doc(inline)]
pub use dog_tv_sim as sim;
#[doc(inline)]
pub use dog_tv_viewer as viewer;

pub use eframe;
pub use sophus;
pub use sophus::nalgebra;
pub use sophus::ndarray;
