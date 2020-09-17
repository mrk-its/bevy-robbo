use bevy::input::keyboard::{ElementState, KeyCode, KeyboardInput};
use bevy::prelude::*;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// Configures an App to run its [Schedule](bevy_ecs::Schedule) according to a given [RunMode]
#[derive(Default)]
pub struct WasmRunnerPlugin;

impl Plugin for WasmRunnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.set_runner(move |mut app: App| {
            let event_handler =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::Event| {
                    match e.type_().as_ref() {
                        "animation_frame" => {
                            app.update();
                        }
                        "keydown" | "keyup" => {
                            let state = if e.type_() == "keydown" {
                                ElementState::Pressed
                            } else {
                                ElementState::Released
                            };
                            let keyboard_event = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();

                            debug!(
                                "event: {:?} {:?} {:?}",
                                keyboard_event.type_(),
                                keyboard_event.code(),
                                keyboard_event
                            );

                            let key_code = Some(match keyboard_event.code().as_ref() {
                                "ArrowLeft" => KeyCode::Left,
                                "ArrowRight" => KeyCode::Right,
                                "ArrowUp" => KeyCode::Up,
                                "ArrowDown" => KeyCode::Down,
                                "ShiftLeft" => KeyCode::LShift,
                                "ShiftRight" => KeyCode::RShift,
                                "PageUp" => KeyCode::PageUp,
                                "PageDown" => KeyCode::PageDown,
                                _ => return,
                            });
                            let mut keyboard_input_events =
                                app.resources.get_mut::<Events<KeyboardInput>>().unwrap();
                            keyboard_input_events.send(KeyboardInput {
                                key_code,
                                state,
                                scan_code: 0,
                            })
                        }
                        _ => return,
                    }
                })
                    as Box<dyn FnMut(web_sys::Event)>);
            let window = web_sys::window().expect("should have a window in this context");
            for &event_type in &["keydown", "keyup", "animation_frame"] {
                window
                    .add_event_listener_with_callback(
                        event_type,
                        event_handler.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }

            let f = Rc::new(RefCell::new(None));
            let g = f.clone();

            *g.borrow_mut() = Some(wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                request_animation_frame(f.borrow().as_ref().unwrap());
                let event = web_sys::CustomEvent::new_with_event_init_dict(
                    "animation_frame",
                    web_sys::CustomEventInit::new().bubbles(true),
                )
                .unwrap();
                window.dispatch_event(&event).unwrap();
            })
                as Box<dyn FnMut()>));
            request_animation_frame(g.borrow().as_ref().unwrap());

            event_handler.forget();
        });
    }
}
