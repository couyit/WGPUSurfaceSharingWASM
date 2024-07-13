use raw_window_handle::RawDisplayHandle;
use raw_window_handle::RawWindowHandle;
use raw_window_handle::WebCanvasWindowHandle;
use raw_window_handle::WebDisplayHandle;
use serde::de::value;
use serde::Deserialize;
use serde::Serialize;
use wgpu::Backends;
use wgpu::InstanceDescriptor;
use std::os::raw::c_void;
use std::ptr::NonNull;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::Surface;
use wgpu::SurfaceTarget;

use wasm_bindgen::JsValue;


#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub unsafe fn create_surface_from_canvas(canvas: &HtmlCanvasElement) -> String {
    web_sys::console::log_1(&"called wasm".into());

    let obj: NonNull<c_void> = NonNull::from(&canvas).cast();

    let window_handle = raw_window_handle::WebCanvasWindowHandle::new(obj).into();
    let display_handle = raw_window_handle::WebDisplayHandle::new().into();

    let instance = wgpu_core::instance::Instance::new("UNPOKO", InstanceDescriptor::default());
    let global = wgpu_core::global::Global::from_instance(instance);

    // let surface = instance.create_surface(SurfaceTarget::Canvas(canvas)).unwrap();
    // let id = surface.global_id();

    let surface_id = global
        .instance_create_surface(display_handle, window_handle, None)
        .unwrap();

    let (index, epoch, backend) = surface_id.unzip();

    let adapter_id = global.request_adapter(
        &wgpu_core::instance::RequestAdapterOptions {
            power_preference: wgpu_types::PowerPreference::default(),
            compatible_surface: Some(surface_id),
            force_fallback_adapter: false,
        },
        wgpu_core::instance::AdapterInputs::Mask(Backends::all(), |_| None),
    ).unwrap();

    web_sys::console::log_1(&format!("{:?}", surface_id).into());

    web_sys::console::log_1(&"The end of surface creation".into());

    serde_json::to_string(&surface_id).unwrap()

    // String::from("Failed")
}
