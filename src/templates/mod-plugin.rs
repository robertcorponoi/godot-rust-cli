use gdnative::api::EditorPlugin;
use gdnative::nativescript::user_data;

#[derive(gdnative::NativeClass)]
#[inherit(EditorPlugin)]
#[user_data(user_data::LocalCellData<MODULE_NAME>)]
pub struct MODULE_NAME;

#[gdnative::methods]
impl MODULE_NAME {
  fn new(_owner: &EditorPlugin) -> Self {
    MODULE_NAME
  }

  #[export]
  fn _ready(&self, _owner: &EditorPlugin) {
    gdnative::godot_print!("hello, world.");
  }
}
