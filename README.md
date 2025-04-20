 # SQLite Diesel WASM Example

An example project demonstrating how one can integrate:

- [SQLite](https://sqlite.org/)
- [Diesel](https://diesel.rs/)
- [Web Assembly](https://webassembly.org/)

Such that you could use Rust to define the core behavior of your app,
and only use Javascript for the frontend.
This may also be useful to define a common Rust core for a multi-platform app,
where you could link against a platform-appropriate SQLite,
but I haven't tested that end-to-end.

## Usage

Requires:

- [Rust](https://www.rust-lang.org/)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- [just](https://github.com/casey/just)
  _(This is used to run commands more easily.
  You could just as well copy the commands out of the `justfile`)_
- [Python](https://www.python.org/)
  _(This is only used to host a local webserver.
  Any webserver would suffice.)_

```shell
just build  # build the WASM+JS files
just serve  # serve the files
```

## License

[MIT Open Source License](./LICENSE)
