# bar

This is a small Rust program that uses [unixbar] to generate some status info. It is used by
[awesome] to show audio volume, battery level, and weather info.

<img src="https://raw.githubusercontent.com/wezm/bar/master/screenshot.png" alt="Screen show of bar in action" width="351" />

See my [rc.lua] for how it's used from awesome.

## Dependencies

### FreeBSD

* dbus
* pkgconfig
* xcb
  * python3 :(

[unixbar]: https://github.com/myfreeweb/unixbar
[awesome]: https://awesomewm.org/
[rc.lua]: https://github.com/wezm/dotfiles/blob/3e773263c0222a8b956923164e8e08438865cd55/config/awesome/rc.lua#L215-L227
