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
[rc.lua]: https://github.com/wezm/dotfiles/blob/c47d5df883b7d1b53478113cc7a57af48e045797/config/awesome/rc.lua#L214-L225
