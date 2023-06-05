# Notes


## Todo
- view root directory in explorer
- right click for actions
- rename
- delete
- create file / dir
- handle relative / absolute path
- impl of notify events




## Goal
Provide a fast, simple tool for taking note, yet powerfull. Must integrate well with new cosmic DE. Code must be as modulable as possible.
- file explorer
- search engine
- tabs
- buttons to launch custom script (like `git pull`)
- support for various formats (pdf, md, html, txt)


## Rust
- [book](https://doc.rust-lang.org/book/ch00-00-introduction.html)

## Iced
- [github](https://github.com/iced-rs/iced)

## notify
- [github](https://github.com/notify-rs/notify)
- [doc](https://docs.rs/notify/6.0.0/notify/)

## cosmic
- [epoch](https://github.com/pop-os/cosmic-epoch)
- [text](ttps://github.com/pop-os/cosmic-text)
- [text-editor](https://github.com/pop-os/cosmic-text-editor)
- [time](https://github.com/pop-os/cosmic-time)

## search engine
- [seroost](https://github.com/tsoding/seroost)


## UI
UI stucture will look something like [this](./asset/app.pdf).
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
