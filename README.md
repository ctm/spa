# Multiple Windows or Single Page for Web and Tauri

This a proof-of-concept for a way of writing an app
to have four different ways of handling windows:
* web_sys multi-window
* web_sys single-page application
* Tauri Desktop (each window is a separate Tauri WebviewWindow)
* Tauri Mobile

## Probably uninteresting to you

The code is convoluted because it started with some code from
(closed source) [mb2](https://ctm.github.io/docs/players_manual/),
which has an unreleased Tauri implementation that works for Tauri
Desktop. I want to extend that code to work with Tauri Mobile, but
that requires me to use a Single-Page Application style rather than
having separate windows for the lobby and each table a player is
playing on or observing.

The source to mb2 is large enough for compile times to be sufficiently
slow that I decided to make this proof-of-concept app to test what I
thought would be a technique to allow me to have both multi-window
and single-page implementation with relatively few source changes.

Regardless of how it's invoked, this code brings up a "lobby" window
that has a "Create Table" button that creates new windows which can
closed by clicking on a wastebasket icon in the lobby. In multi-window
mode (the default except for iOS or Android), each window can also be
closed by using the native close button for the window that's created,
whether it's a web window or a Tauri application window.

In single-page mode, there is only a single-page that shows either the
lobby or a single table, but the table windows have a button to get
you back to the lobby. Additionally, each table window has a left and
right arrow to get you to a lesser or greater table (based on id), if
one exists. In the lobby, each table name is itself a link to bring
you to that table.

I have no idea how you found this repository, but it's probably not
interesting to you.  If it is, however, great.

## To run as a multi-page web app

```
trunk serve
```
and then visit http://localhost:1420

### To turn the `spa` (single-page) feature on, include `--features=spa`
```
trunk serve --features=spa
```

## To run as a Tauri multi-page app
```
cargo tauri dev
```

### To run as a Tauri single-page app
```
cargo tauri dev -c src-tauri/spa.conf.json
```

## iOS and Android automatically have the spa feature enabled

Both
```
cargo tauri ios dev [--host]
```
and
```
cargo tauri android dev
```
automatically enable the `spa` feature, as do the  `build` variants.
