# Pomodoro Timer

![Pomodoro Timer Screenshot]()

A simple, beautiful, and resource-efficient Pomodoro timer built with Rust and the [Slint](https://slint-ui.com/) UI toolkit.

This application is fully cross-platform and is automatically built for Windows, macOS, and Linux on every new release.

## Features

-   Standard Pomodoro cycles: Work, Short Break, and Long Break.
-   Start, pause, and reset the timer.
-   Skip the current session to move to the next one.
-   Clean, minimalist user interface.
-   Low memory and CPU usage, especially in the release build.

## Downloading the Application

The easiest way to get the application is to download a pre-built version for your operating system from the **[Releases](https://github.com/reRumbl/LightweightPomodoroTimer/releases)** page.

> **Note for Windows users:**
> Windows SmartScreen might show a warning because the application is not digitally signed. This is expected for small, independent projects. You will need to click "More info" -> "Run anyway" to start the application.

## Building from Source

If you prefer to build the application yourself, you'll need to have the Rust toolchain installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/reRumbl/LightweightPomodoroTimer.git
    cd LightweightPomodoroTimer
    ```

2.  **Install dependencies (for Linux only):**
    ```bash
    sudo apt-get update && sudo apt-get install -y libfontconfig1-dev libgtk-3-dev
    ```

3.  **Build and run the application:**

    For a quick debug build:
    ```bash
    cargo run
    ```

    For a fully optimized release build (recommended for regular use):
    ```bash
    cargo run --release
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.