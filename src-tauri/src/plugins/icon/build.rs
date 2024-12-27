const COMMANDS: &[&str] = &["get_icon"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
