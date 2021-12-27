use wasm_bindgen::prelude::*;
use winit::{
    event::{DeviceEvent, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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

fn any_log(msg: &str) {
    println!("{}", msg);
    #[cfg(target_arch = "wasm32")]
    log(msg);
}

#[wasm_bindgen(start)]
pub fn open_window() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let event_loop = EventLoop::new();

    #[cfg(not(target_arch = "wasm32"))]
    let window = WindowBuilder::new()
        .with_title("Mouse Wheel events")
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    println!("started with window size: {:?}", window.inner_size());

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
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        any_log(format!("mouse wheel Line Delta: ({},{})", x, y).as_str());
                    }
                    winit::event::MouseScrollDelta::PixelDelta(p) => {
                        any_log(format!("mouse wheel Pixel Delta: ({},{})", p.x, p.y).as_str());
                    }
                },
                WindowEvent::MouseInput { button, .. } => match button {
                    winit::event::MouseButton::Left => {
                        any_log("left click");
                    }
                    winit::event::MouseButton::Right => {
                        any_log("right click");
                    }
                    winit::event::MouseButton::Middle => {
                        any_log("middle click");
                    }
                    winit::event::MouseButton::Other(val) => {
                        any_log(&format!("other click {}", val));
                    }
                },
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => match keycode {
                    VirtualKeyCode::Space => {
                        any_log(format!("Space: {:?}", state).as_str());
                    }
                    _ => (),
                },

                _ => {}
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    if LOG_MOUSE_MOTION {
                        any_log(
                            format!("mouse motion Pixel Delta: ({},{})", delta.0, delta.1).as_str(),
                        );
                    }
                }
                _ => (),
            },
            _ => {}
        }
    });
}
