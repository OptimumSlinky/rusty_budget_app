// This effectively makes the Slint code a macro and allows you to combine the UI and business logic
// At some point I'll probably move this to its own .slint file but this is fine for now
slint::slint!{
    import { LineEdit } from "std-widgets.slint";
export component AppWindow inherits Window{
        LineEdit {
            font-size: 14px;
            width: 200px;
            height: 25px;
            input-type: number;
            placeholder-text: "Enter income here.";
        }
    }  
}

// Main app loop
fn main() {
    AppWindow::new().unwrap().run().unwrap(); // That's not ugly or anything
}
