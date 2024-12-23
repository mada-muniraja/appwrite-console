# Appwrite Console Cross-Platform App

This is an unofficial cross-platform app for [Appwrite Console](https://cloud.appwrite.io/) made with [Tauri](https://tauri.app/).

The app is a simple wrapper around the Appwrite console web interface, allowing you to access the console from a native application on any operating system.

## Build the app using source

### Clone the repository to a local directory

```
$ git clone https://github.com/mada-muniraja/appwrite-console.git && cd appwrite-console
```

### Install the Tauri CLI tool

```
$ cargo install tauri-cli
```

### Build the Tauri Dektop application

```
$ cargo tauri build
```

### Build the Tauri Android apk

```
$ cargo tauri android build --apk
```

### Build the Tauri Android App Bundle

```
$ cargo tauri android build --aab
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
