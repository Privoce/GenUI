use gen_compiler::{app, Target, Builder};

fn main() {
    let compiler = Target::makepad()
        .entry("app")
        .root("E:/Rust/try/makepad/Gen-UI/examples/gen_ark_makepad/ui/views/root.gen")
        .add_dep("makepad-widgets")
        .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
        .build()
        // .wasm()
        // .build()
        .build();
   
    // set app and specify target
    let mut app = app(Some(Box::new(compiler))).build();
    let _ = app.run();
}
