// This effectively makes the Slint code a macro and allows you to combine the UI and business logic
// At some point I'll probably move this to its own .slint file but this is fine for now
slint::slint!{
    import { LineEdit, Slider } from "std-widgets.slint";
export component AppWindow inherits Window{
        width: 320px;
        height: 240px;
    VerticalLayout {
        spacing: 5px;

        // User data entry point
        LineEdit {
            horizontal-alignment: center;
            font-size: 14px;
            width: 200px;
            height: 25px;
            input-type: number;
            placeholder-text: "Enter income here.";
        }
    
        // 
        Text {
            text: "Housing";
        }

        Slider {
            width: parent.width * 80%;
            height: parent.height;
            value: 25;
            minimum: 0;
            maximum: 100;
        }
    }
}  
}

// Main app loop
fn main() {
    AppWindow::new().unwrap().run().unwrap(); // That's not hideous at all...
}
