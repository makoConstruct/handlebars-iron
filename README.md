handlebars-iron
===============

[Handlebars](https://github.com/sunng87/handlebars-rust) middleware
for the [Iron web framework](http://ironframework.io).

[![Build
Status](https://travis-ci.org/sunng87/handlebars-iron.svg?branch=master)](https://travis-ci.org/sunng87/handlebars-iron)
[![](http://meritbadge.herokuapp.com/handlebars-iron)](https://crates.io/crates/handlebars-iron)
[![Gitter](https://img.shields.io/gitter/room/sunng87/handlebars-rust.svg?maxAge=2592000)](https://gitter.im/sunng87/handlebars-rust)

The most recent version of handlebars-iron, like Hyper, Iron and
Handlebars-rust, now compiles on nightly, beta and latest stable (1.7.0+) channel. Our
travis task will track the compatibility on all these channels.

Both iron and handlebars has backward-incompatible change during 0.x
releases. So you will need to choose handlebars-iron version based on
those two versions you were using:

handlebars-iron | handlebars | iron
--------------- | ---------- | ---
0.14.x | 0.16.x | 0.2.x
0.15.x | 0.18.x | 0.3.x
0.16.0 | 0.19.x | 0.3.x
0.17.x | 0.19.x | 0.4.x
0.18.x | 0.20.x (serde 0.8) | 0.4.x

## Usage

Add HandlebarsEngine to your Iron middleware chain as an "after"
middleware.

```rust
  /// HandlebarsEngine will look up all files with "./examples/templates/**/*.hbs"
  let mut hbse = HandlebarsEngine::new();
  hbse.add(Box::new(DirectorySource::new("./examples/templates/", ".hbs")));

  // load templates from all registered sources
  if let Err(r) = hbse.reload() {
    panic!("{}", r);
  }

  chain.link_after(hbse);
```

If you want register your own custom helpers, you can initialize the
`HandlebarsEngine` from a custom `Handlebars` registry.

```
  /// HandlebarsEngine will look up all files with "./examples/templates/**/*.hbs"
  let mut hb = Handlebars::new();
  hb.register_helper("helper", my_helper);

  let mut hbse = HandlebarsEngine::from(hb);
  hbse.add(Box::new(DirectorySource::new("./examples/templates/", ".hbs")));

  // load templates from all registered sources
  if let Err(r) = hbse.reload() {
    panic!("{}", r);
  }

  chain.link_after(hbse);
```

You can find more information about custom helper in [handlebars-rust
document](https://github.com/sunng87/handlebars-rust#extensible-helper-system).

In your handler, set `Template` to response. As required by
Handlebars-rust, your data should impl `serialize::json::ToJson`. If
you are on nightly channel, it is highly recommended to use
[tojson_macros](https://github.com/sunng87/tojson_macros) to generate
default `ToJson` implementation without repeating yourself.

For `DirectorySource`, handlebars engine will walk the directory
specified by `prefix`, try to register all templates matches the
suffix, and extract its name as template name. For instance,
`./examples/templates/some/path/index.hbs` will be registered as
`some/path/index`.

```rust
/// render data with "index" template
/// that is "./examples/templates/index.hbs"
fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = ...
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}
```

By using `Template::with` You can also render some template without
actually register it. But this is not recommended because template
string needs to be parsed every time. Consider using a `MemorySource`
if possible.

```rust
/// render data with "index" template
/// that is "./examples/templates/index.hbs"
fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = ...
    resp.set_mut(Template::with("<h1>{{title}}</h1>", data)).set_mut(status::Ok);
    Ok(resp)
}
```

Since this is simple library, you may run this
[example](https://github.com/sunng87/handlebars-iron/blob/master/examples/server.rs)
with `RUST_LOG=handlebars_iron=info cargo run --example server`
first, and
[documentation](https://sunng87.github.io/handlebars-iron/handlebars_iron/)
then.

Rust and its ecosystem are still in early stage, this
project might been broken for various reasons. I will try my best to
keep this library compiles with latest Rust nightly before the 1.0
final release. If you find anything bad, pull requests and issue reporting
are always welcomed.

## Live reload

During development you may want to live-reload your templates without
having to restart your web server. Here comes the live-reload
feature.

Since live-reload may only be useful in development phase, we have
made it a optional feature. In order to enable it, you will need to
add feature `watch` in your cargo declaration:

```toml
[features]
## create a feature in your app
watch = ["handlebars-iron/watch"]

[dependencies]
handlebars-iron = ...
```

Check `examples/watch_server.rs` for further information. To test it:
`RUST_LOG=handlebars_iron=info cargo run --example watch_server
--features watch`.

## Serde

From 0.11.0, by enabling feature `serde_type`, you can also use
[serde_json](https://github.com/serde-rs/json) as type
system. Typically you can annotate your type with
`#[derive(Serialize)]` to make it work as template data.

You can find an working example in `examples/server.rs` and try to run
it via: `cargo run --features unstable --example server
--no-default-features`

## Using handlebars-iron?

Add your project to our
[adopters](https://github.com/sunng87/handlebars-rust/wiki/adopters).

## License

MIT, of course.
