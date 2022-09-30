// https://slint-ui.com/releases/0.3.0/docs/rust/slint/docs/builtin_elements/index.html#path-using-svg-path-elements

fn main() {
    MainWindow::new().run();
}

slint::slint! {
    MainWindow := Window {
        preferred-width: 400px;
        preferred-height: 400px;

            Path {
                width: 100px;
                height: 100px;
                commands: "M 0 0 L 0 100 A 1 1 0 0 0 100 100 L 100 0 Z";
                stroke: red;
                stroke-width: 1px;
            }

            Path {
                width: 200px;
                height: 200px;
                commands: "M 200 0 L 100 30 L 150 50 Z";
                stroke: black;
                stroke-width: 3px;
            }
    }
}
