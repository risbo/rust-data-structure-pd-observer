use editor::Editor
use observer::Event

mod editor
mod observer

/**
 * Observer Pattern
 * Is a behavioral design pattern that allows some objects to notify
 * other objects about changes in their state.
 * 
 * The Observer pattern provides a way to subscribe and unsubscribe to and these events
 * for any object that implements a subscriber interface
 * 
 * Factoring Guru. (2024). Observer in rust
 * https://refactoring.guru/design-patterns/observer/rust/example

 */
fn main(){
    
    let mut editor = Editor::default();

    editor.events().subscribe(Event::Load, |file_path| {
        let log = "/path/to/log/file.text".to_string();
        println!("Save log to {}: Load file {}", log, file_path);
    });

    editor.events().subscribe(Event::Save, save_listener);

    editor.load("text1.txt".into());
    editor.load("text2.txt".into());
    editor.save();
}

fn save_listener(file_path: String){
    let email = "bpalaciosa@gmail.com".to_string;
    println!("Email to {}:Save file {}", email, file_path);
}