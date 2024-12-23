# Appwrite Console Desktop App

This is an unofficial desktop app for [Appwrite Console](https://cloud.appwrite.io/) made with [Tauri](https://tauri.app/).

The app is a simple wrapper around the Appwrite console web interface, allowing you to access the console from a native desktop application.

![Appwrite project dashboard showing various Appwrite features](./resources/github.png)

## Build the app from source

### Setup your system for Tauri

Please check this URL: https://tauri.app/start/prerequisites/

### Clone the repository

```
$ git clone https://github.com/mada-muniraja/appwrite-console.git && cd appwrite-console
```

### Install the Tauri CLI tool

```
cargo install tauri-cli
```

### Build Dektop application

```
cargo tauri build
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
