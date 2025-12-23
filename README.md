# bevy_html_tailwind

## Features

- [x] Hot-reloading
- [x] Loading images
- [x] Using HTML id as a marker component
- [ ] Custom fonts

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

## Compatibility with bevy

| Bevy Version | bevy_html_tailwind |
|--------------|--------------------|
| 0.17         | 0.1                |
