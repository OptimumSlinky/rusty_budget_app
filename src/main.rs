// This effectively makes the Slint code a macro and allows you to combine the UI and business logic
// At some point I'll probably move this to its own .slint file but this is fine for now
slint::slint!{
    export component AppWindow {
        Text {
            text: "radio check";
            color: blue;
        }
    }  
}

// Main app loop
fn main() {
    AppWindow::new().unwrap().run().unwrap(); // That's not ugly or anything
}
