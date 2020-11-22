# Bevy Robbo

Rust port of great 8-bit [Atari game](https://en.wikipedia.org/wiki/Robbo_(video_game)). It is created with great [Bevy](https://github.com/bevyengine/bevy) game engine and may be built as native (for Linux, Windows, Mac) or [web application](https://mrk.sed.pl/bevy-robbo/). Web version uses WebGL2 rendering thanks to [bevy_webgl2](https://github.com/mrk-its/bevy_webgl2) rendering plugin.

Game uses graphics / level data from [GNU Robbo](http://gnurobbo.sourceforge.net)

![Robbo Screenshot](https://mrk.sed.pl/bevy-showcase/assets/bevy_robbo.png)

## Build instructions

### Prerequisites

* [rust](https://www.rust-lang.org/tools/install)
* [cargo-make](https://github.com/sagiegurari/cargo-make#installation)

### Building and running native version
```
$ cargo make run
```

### Building and running web version
```
$ cargo make serve
```
and point your web browser to [http://localhost:4000/](http://localhost:4000/)

### How to play

Move with arrows, shot with shift + arrow, reset level with Esc

Enyoy!
------
