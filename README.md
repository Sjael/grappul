# <img src="static/logo.svg" height="60">

A modern web application built with Rust and Dioxus for exploring and learning about gods, abilities, and items in gaming. Grappul provides an interactive interface for viewing character builds, abilities, and strategic information.

## Features

- 🎮 Comprehensive god information and abilities
- 🛠️ Item builds and recommendations
- 📊 Cheatsheets for quick reference
- 🎯 Role and class filtering
- 💡 Detailed tooltips and explanations
- 🖼️ High-quality ability and item icons

## Technology Stack

- **Frontend Framework**: [Dioxus](https://dioxuslabs.com/) - A React-like framework for Rust
- **Language**: [Rust](https://www.rust-lang.org/)
- **Styling**: CSS
- **Asset Management**: Static file serving for images and icons

## Project Structure

```
grappul/
├── src/
│   ├── components/     # Reusable UI components
│   ├── data/          # Game data and JSON files
│   ├── routes/        # Application routes
│   └── utils.rs       # Utility functions
├── assets/
│   ├── abilities/     # Ability icons
│   ├── gods/         # God portraits
│   ├── icons/        # UI icons
│   └── items/        # Item icons
└── Cargo.toml         # Rust dependencies
```

## Getting Started

1. **Prerequisites**
   - Install Rust and Cargo
   - Install Dioxus CLI

2. **Installation**
   ```bash
   # Clone the repository
   git clone https://github.com/yourusername/grappul.git
   cd grappul

   # Install Dioxus CLI
   cargo install dioxus-cli
   ```

3. **Development**
   ```bash
   # Start the development server
   dx serve
   ```

4. **Building**
   ```bash
   # Create a production build
   dx build --release
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
