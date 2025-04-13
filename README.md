# Getting Started with Rust

Rust is a systems programming language focused on safety, speed, and concurrency. This guide will help you set up your environment and start building Rust applications using Visual Studio Code on Windows.

## Setting Up Your Environment

### Prerequisites
1. Install Rust using `rustup`:
   Open a terminal and run:
   ```bash
   winget install -e --id Rustlang.Rustup
   ```

2. Install Visual Studio Code:
   ```bash
   winget install -e --id Microsoft.VisualStudioCode
   ```

3. Install the following VS Code extensions:
   - [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
   - [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) (for debugging)

   You can install these extensions directly from the Extensions view in VS Code (`Ctrl+Shift+X`).

## Creating a New Rust Project

1. Create a new project using `cargo`:
   ```bash
   cargo new my_project
   cd my_project
   ```

2. The project structure will look like this:
   ```
   my_project/
   ├── Cargo.toml  # Project metadata and dependencies
   └── src/
       └── main.rs  # Entry point of the application
   ```

## Building and Running Your Project

1. Build the project:
   ```bash
   cargo build
   ```

2. Run the application:
   ```bash
   cargo run
   ```

3. Run tests:
   ```bash
   cargo test
   ```

## Debugging Your Rust Application

1. Configure debugging in VS Code:
   - Create a `.vscode/launch.json` file:
     ```json
     {
       "version": "0.2.0",
       "configurations": [
         {
           "name": "Debug Rust",
           "type": "lldb",
           "request": "launch",
           "program": "${workspaceFolder}/target/debug/my_project",
           "args": [],
           "cwd": "${workspaceFolder}",
           "stopOnEntry": false,
           "sourceLanguages": ["rust"]
         }
       ]
     }
     ```

2. Press `F5` in VS Code to start debugging.

## Additional Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
