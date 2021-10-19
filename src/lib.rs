use wasm_bindgen::prelude::*;
use winit::{event::{DeviceEvent, Event, KeyboardInput, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowBuilderExtWebSys;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Easily remove the motion noise from the log...
const LOG_MOUSE_MOTION: bool = true;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn open_window() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let event_loop = EventLoop::new();

    //#[cfg(not(target_arch = "wasm32"))]
    //{
    #[cfg(not(target_arch = "wasm32"))]
    let window = WindowBuilder::new()
        .with_title("Mouse Wheel events")
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    println!("started with window size: {:?}", window.inner_size());
    //}

    // Get the browser window.
    #[cfg(target_arch = "wasm32")]
    let dom_window = web_sys::window().expect("Can not get browser window.");

    // Get the dom document.
    #[cfg(target_arch = "wasm32")]
    let document = dom_window.document().expect("Can not get html document.");

    // Get the canvas with the given id.
    #[cfg(target_arch = "wasm32")]
    let canvas = document
        .get_element_by_id("my_canvas")
        .expect("The given canvas id was not found in the document.");

    // Canvas need to be of type HtmlCanvasElement.
    #[cfg(target_arch = "wasm32")]
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Failed to convert element to canvas.");

    #[cfg(target_arch = "wasm32")]
    let window = WindowBuilder::new()
        .with_title("Mouse Wheel events")
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    log(format!("started with window size: {:?}", window.inner_size()).as_str());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseWheel { delta } => match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        println!("mouse wheel Line Delta: ({},{})", x, y);
                        #[cfg(target_arch = "wasm32")]
                        log(format!("mouse wheel Line Delta: ({},{})", x, y).as_str());
                    }
                    winit::event::MouseScrollDelta::PixelDelta(p) => {
                        println!("mouse wheel Pixel Delta: ({},{})", p.x, p.y);
                        #[cfg(target_arch = "wasm32")]
                        log(format!("mouse wheel Pixel Delta: ({},{})", p.x, p.y).as_str());
                    }
                },
                DeviceEvent::Button {
                    button: 1, // The Left Mouse Button.
                    state,
                } => {
                    println!("mouse Button: {:?}", state);
                    #[cfg(target_arch = "wasm32")]
                    log(format!("mouse Button: {:?}", state).as_str());
                }
                DeviceEvent::Key(KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                }) => match keycode {
                    VirtualKeyCode::Space => {
                        println!("Space: {:?}", state);
                        #[cfg(target_arch = "wasm32")]
                        log(format!("Space: {:?}", state).as_str());
                    }
                    _ => (),
                },
                // This is the only event that is fired on WASM.
                DeviceEvent::MouseMotion { delta } => {
                    if LOG_MOUSE_MOTION {
                        println!("mouse motion Pixel Delta: ({},{})", delta.0, delta.1);
                        #[cfg(target_arch = "wasm32")]
                        log(
                            format!("mouse motion Pixel Delta: ({},{})", delta.0, delta.1).as_str(),
                        );
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}
