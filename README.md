# Single Page Application

This is just a toy program to allow me to test a way of writing an app
to have three different ways of handling windows:
* web_sys (each window is a separate browser window)
* Tauri Desktop (each window is a separate Tauri WebviewWindow)
* Tauri (only one window, but it can switch between multiple views)

## Probably uninteresting to you

The first several commits are just me using cargo create-tauri-app and
then doing some configuration to make it work with the various
platforms that Tauri supports.  After that I've added code copied in
from mb2 (a private repository) that currently supports web_sys and
Tauri Desktop windows.

Spa brings up a "lobby" window that has a "Create Table" button that
creates pop-up windows which can either be closed by clicking on a
wastebasket icon or by closing the window natively.

Eventually, I'll add a single-page-application paradigm, where there's
only one window, which can shift between the lobby and the various
tables.  Once that's working, I'll apply the technique to mb2 so that
the iOS and Android builds can use it.

I have no idea how you found this repository, but it's probably not
interesting to you.  If it is, however, great.

## To run as a web app

```
trunk serve
```
and then visit http://localhost:1420

## To run as a Tauri app
```
cargo tauri dev
```
