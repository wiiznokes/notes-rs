# [Notes](https://github.com/wiiznokes/notes-rs)

Now in https://github.com/wiiznokes/note

## Todo

- [x] view root directory in explorer
- [x] right click for actions
- [x] rename
- [x] delete
- [x] handle relative / absolute path
- [x] impl of notify events
- [x] open dir from app
- [x] fix explorer number display (11 before 100)
- [ ] settings with [serde](https://github.com/serde-rs/serde) in [toml](https://github.com/toml-rs/toml)
- [ ] config file for shared and local with [directories](https://github.com/dirs-dev/directories-rs)
- [ ] unwatch all directory in notify
- [ ] create file / dir
- [ ] open file from app

## Goal

Provide a fast, simple tool for taking note, yet powerful. Must integrate well with new cosmic DE. Code must be as
modular as possible.

- file explorer
- search engine
- tabs
- buttons to launch custom script (like `git pull`)
- support for various formats (pdf, md, html, txt)
- Settings

## Rust

- [book](https://doc.rust-lang.org/book/ch00-00-introduction.html)

## Iced

- [github](https://github.com/iced-rs/iced)
- [doc](https://docs.rs/iced/latest/iced/)
- [aw](https://github.com/iced-rs/iced_aw)

## notify

- [github](https://github.com/notify-rs/notify)
- [doc](https://docs.rs/notify/6.0.0/notify/)

## cosmic

- [epoch](https://github.com/pop-os/cosmic-epoch)
- [text](https://github.com/pop-os/cosmic-text)
- [text-editor](https://github.com/pop-os/cosmic-text-editor)
- [time](https://github.com/pop-os/cosmic-time)

## search engine

- [seroost](https://github.com/tsoding/seroost)

file picker
https://github.com/PolyMeilex/rfd

## UI

UI structure will look something like [this](./asset/app.pdf).
And will be very inspired of [text-editor](https://github.com/pop-os/cosmic-text-editor).

## Dependencies Fedora

- for iced

```
sudo dnf install freetype-devel expat-devel fontconfig-devel
```

- for cosmic (not used in here)

```
sudo dnf install rust-gdk-sys+default-devel
```

## icons

- https://icons.getbootstrap.com/
- https://icons.getbootstrap.com/icons/file-earmark/
- https://icons.getbootstrap.com/icons/folder/
- https://icons.getbootstrap.com/icons/chevron-down/
- https://icons.getbootstrap.com/icons/chevron-right/


### autofix
```
cargo clippy --all --fix --allow-dirty --allow-staged && cargo fmt --all
```
