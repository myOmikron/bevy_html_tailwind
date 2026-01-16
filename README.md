# bevy_html_tailwind

[![Crates.io License](https://img.shields.io/crates/l/bevy_html_tailwind)](https://github.com/myOmikron/bevy_html_tailwind/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/bevy_html_tailwind)](https://crates.io/crates/bevy_html_tailwind)
[![crates io downloads](https://img.shields.io/crates/d/bevy_html_tailwind)](https://crates.io/crates/bevy_html_tailwind)
[![docs.rs](https://img.shields.io/docsrs/bevy_html_tailwind)](https://docs.rs/ikebuster/latest/bevy_html_tailwind/)
[![ci](https://img.shields.io/github/actions/workflow/status/myOmikron/bevy_html_tailwind/ci.yml?label=CI)](https://github.com/myOmikron/bevy_html_tailwind/actions/workflows/ci.yml)

## Features

- [x] Hot-reloading
- [x] Loading images
- [x] Using HTML id as a marker component
- [x] Custom fonts
- [ ] Rewrite parser code
- [ ] Support percentage values everywhere

## Setup

```rust
use bevy::prelude::*;
use bevy_html_tailwind::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HtmlTailwindPlugin::default())
        .add_systems(Startup, load_ui)
        .run();
}

fn load_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(HtmlTailwindBundle {
        ui: asset_server.load("menu.html").into(),
        ..Default::default()
    })
}
```

In the corresponding `assets` directory, place your HTML file `menu.html`.
You can now use tailwind CSS for styling your nodes!

```html

<div class="flex flex-col w-full h-full justify-center items-center gap-[20px]">
    <button>Start</button>
    <button>Exit</button>
</div>
```

**Important!**

Only one root node is allowed!

## Hot-reloading

Hot-reloading is supported for HTML files. When you make changes to your HTML files, the changes will be automatically
reflected in your game without needing to restart the application. To use this, enable the `file_watcher` feature
of bevy.

## Loading images

Loading images is supported by the `bevy_html_tailwind` crate. You can load images in your HTML files using the `<img>`
tag and specifying the image path relative to your assets' directory. For example:

```html
<img src="images/example.png"/>
```

## Custom fonts

To use custom fonts, we use an html-like structure with a `<head>` section for the definition
of the fonts. Their `src` must be a valid path in your assets' directory.

The first decendent of the `<body>` section will be used as your root node displaying-wise.

```html

<html>
<head>
    <!-- This node will be treated as root node -->
    <font name="pixelify" src="fonts/pixelify.ttf"/>

    <!-- Default fonts can also be set with this method -->
    <font name="default" src="fonts/pixelify.ttf"/>
    <!-- or -->
    <font src="fonts/pixelify.ttf"/>
</head>
<body>

<!-- This node will be treated as root node -->
<div class="flex flex-col justify-center items-center w-full h-full">
    <span class="font-pixelify">Heading with custom font</span>
    <span>Custom default font</span>
</div>

</body>
</html>
```

## Compatibility with bevy

| Bevy Version | bevy_html_tailwind |
|--------------|--------------------|
| 0.17         | 0.1                |
| 0.18         | 0.2                |

## Contributing

Contributions are welcome!
If you find any issues or have suggestions for improvements, please open an issue or submit a PR.

Make sure to format your code using the provided `.rustfmt.toml`.
(Nightly is currently required: `cargo +nightly fmt`)