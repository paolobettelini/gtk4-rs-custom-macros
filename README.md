# GTK4 Rust Bindings macros

These proc macros are not intended for use

## Import the macros
```rust
use gtk_custom_macros::*;
```
Note that `Box` is called `GTKBox`.

## Builder pattern Macro
Instead of this
```rust
let button = ButtonBuilder::new()
    .label("button1")
    .halign(Center)
    .valign(Center)
    .build();
```
You can do this
```rust
let button = button!(
    label "button1"
    halign Center
    valign Center
);
```

## CSS Macro
Instead of this
```rust
let provider = CssProvider::new();
provider.load_from_data(include_str!("style.css"));
StyleContext::add_provider_for_display(
    &Display::default().expect("Could not connect to a display."),
    &provider,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
);
```
You can do this
```rust
load_css!("style.css");
```

## Builder macro
Instead of this
```rust
let ui_src = include_str!("hello.ui");
let builder = Builder::from_string(ui_src);

let window: ApplicationWindow = builder.object("window").unwrap();
window.set_application(Some(application));

let button: Button = builder.object("button").unwrap();
let dialog: MessageDialog = builder.object("messagedialog").unwrap();
```
You can do this
```rust
let ui = parse_ui!("hello.ui");

ui.window.set_application(Some(application));

let button = ui.button;
let dialog = ui.messagedialog;
```
