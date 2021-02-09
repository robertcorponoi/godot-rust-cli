use gdnative::api::Node2D;
use gdnative::prelude::*;

#[inherit(Node2D)]
#[derive(NativeClass)]
pub struct MODULE_NAME;

#[methods]
impl MODULE_NAME {
    fn new(_owner: &Node2D) -> Self {
        MODULE_NAME {}
    }
    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        godot_print!("Hello world!");
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, _delta: f32) {}
}
