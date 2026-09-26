# Nudl
This is Nudl (Nearly User Defined Language). It's a programming language with customizable syntax that translates into a Rust project and then compiles into native binaries.

## Prerequisites
You must have the Rust toolchain installed for Nudl to work! Since Nudl is simply a translator, you need the Rust toolchain in order to run or build your project. To use Nudl, you do not need to interact with the Rust toolchain directly, Nudl handles that for you.

## Install
Head over to [releases](https://github.com/Mang00oo/Nudl/releases/tag/v0.1.0) and run the shell script in your terminal of choice, or download a binary directly.

## Usage
- To customize your syntax, create a project and edit flavor.json.
> The generated Rust project is in the /translated directory.
```bash
nudl version
```
- Gets the current version of your Nudl installation.
```bash 
nudl create my-project
```
- Creates a project in the current directory with the given name
```bash
nudl run
```
- Translates and runs the project
```bash
nudl build
```
- Translates and builds the project to the current platform using the Rust toolchain. Built binaries are found under /translated/target/release
```bash
nudl translate
```
- Translates the project but does not run it or build it

## Language Features
- If/else/else if conditionals
- Functions with parameters and return statents with type inference (can only return bool, float, or string)
- While / for loops with break and continue
- Variables
- Math operators (+, -, *, /, %) and comparisons (and, or, not) evaluated where needed

## Customization Features
- Define all keywords and symbols in flavor.json

### Planned Customization Features
- Choose between statically typed or dynamically typed syntax
- Choose indentation as your block definition, like in Python
- Map files that map certain functions in your code to Rust functions (partially implemented)

### Planned Language Features
- Arrays
- More types
- Classes
- Map files for translating more functions into Rust code
> Currently, maps are partially implemented. The folder and a sample map (std.json) are created when you create a project, but the translator does not use them yet.