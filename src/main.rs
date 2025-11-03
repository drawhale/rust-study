use rust_example::{Button, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 50,
                label: "Click me".to_string(),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 50,
                options: vec!["Option 1".to_string(), "Option 2".to_string()],
            }),
        ],
    };

    screen.run();
}
