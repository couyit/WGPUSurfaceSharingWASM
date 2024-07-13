mod utils;

use raw_window_handle::RawDisplayHandle;
use raw_window_handle::RawWindowHandle;
use raw_window_handle::WebCanvasWindowHandle;
use raw_window_handle::WebDisplayHandle;
use serde::Deserialize;
use serde::Serialize;
use std::os::raw::c_void;
use std::ptr::NonNull;
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct SurfaceIdSerializable {
    pub index: u32,
    pub epoch: u32,
    pub backend: wgpu_types::Backend,
}

#[wasm_bindgen]
pub unsafe fn create_surface_from_canvas(canvas: &JsValue) -> String {
    let obj: NonNull<c_void> = NonNull::from(canvas).cast();
    let handle = WebCanvasWindowHandle::new(obj);

    let window_handle: RawWindowHandle = RawWindowHandle::WebCanvas(handle).into();
    let display_handle: RawDisplayHandle = WebDisplayHandle::new().into();

    let instance = wgpu_core::instance::Instance::default();
    let global = wgpu_core::global::Global::from_instance(instance);

    let surface_id = global
        .instance_create_surface(display_handle, window_handle, None)
        .unwrap();
    let (index, epoch, backend) = surface_id.unzip();

    let serialized = SurfaceIdSerializable {
        index,
        epoch,
        backend,
    };

    serde_json::to_string(&serialized).unwrap()
}
