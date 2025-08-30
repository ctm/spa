# Single Page Application

This is just a toy program to allow me to test a way of writing an app
to have three different ways of handling windows:
* web_sys (each window is a separate browser window)
* Tauri Desktop (each window is a separate Tauri WebviewWindow)
* Tauri (only one window, but it can switch between multiple views)

## Probably uninteresting to you

The first several commits are just me using cargo create-tauri-app and
then doing some configuration to make it work with the various
platforms that Tauri supports.  Eventually, I'll copy in some code
from mb2 (a private repository) that currently supports web_sys and
Tauri Desktop windows, and then add a single-page-application
paradigm.  Once I have that working, I'll then bring the code back
into mb2.

I have no idea how you found this repository, but it's probably not
interesting to you.  If it is, however, great.
