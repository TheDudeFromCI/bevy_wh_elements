<div align="center">
<h1>WhElements</h1>
<p><i><code>bevy_wh_elements</code>, short for <b>Bevy: Wraithaven's Elements</b>, is a proof-of-content user interface wrapper library for the <code>Bevy</code> game engine. The purpose of this library is to add a level of abstraction over the default implementation of the <code>Bevy</code> user interface in order to reduce a significant amount of the boilerplate that is required to get a simple menu operation.</i></p>
</div>

---

***Note:* This project is still considered a proof of concept at this time. As such, there are currently no plans to upload this version of the library to `crates.io` until a cleaner and more well defined version of this codebase are written. This project will also only attempt to maintain the codebase for the latest version of `Bevy`. This package mainly exists for usage within the **Wraithaven** library ecosystem.**

## Getting Started

To install this project from git, simply add the following to your `Cargo.toml`, under your existing dependencies.

```toml
[dependencies]
bevy = "0.12.0"
bevy_wh_elements = { git = "https://github.com/TheDudeFromCI/bevy_wh_elements" }
```

Please note that this will use the bleeding edge version of the codebase, which may contain frequent updates, bugfixes, and API changes. If you wish to lock your version to a specific commit of this library, which is highly recommended for active projects, you can also specify the commit version within your `Cargo.toml` to lock the version in place.

```toml
[dependencies]
bevy = "0.12.0"
bevy_wh_elements = { git = "https://github.com/TheDudeFromCI/bevy_wh_elements", rev = "a636f9e" }
```

## Versions

| Bevy Version | WhElements |
| -----------: | ---------: |
|       0.12.0 |      0.1.0 |

## Additional Credits

This library uses the custom `cursor.ttf` font provided by `bevy_simple_text_input`. As such, usage of that font falls under the license used by that library.
<https://github.com/rparrett/bevy_simple_text_input>

A special thanks to the developers of that plugin for providing this font, as well as inspiration for code snippets used in the development of this library.
