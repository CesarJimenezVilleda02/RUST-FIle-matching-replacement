# Find and Replace Program README

## Overview

This Rust program provides a simple but powerful tool for text processing: it finds and replaces strings in text files. Utilizing regular expressions for pattern matching, it can handle complex search-and-replace operations, making it a versatile tool for editing texts, coding, data manipulation, and more.

## Features

- **Regular Expression Support**: Use regular expressions to define the pattern of text to be replaced, offering flexibility in targeting specific strings.
- **File Operations**: Reads from an input file and writes the modified content to an output file, automating the process of editing large or multiple files.
- **Error Handling**: Provides clear error messages for common issues such as file read/write errors and regex compilation problems.
- **Command Line Interface**: Easily run from the command line, allowing integration into scripts or workflows.

## Requirements

- Rust Programming Language
- Cargo (Rust's package manager and build system)

## Installation

To use this program, first ensure you have Rust and Cargo installed on your system. If not, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to set them up.

Clone the repository or download the source code, then navigate to the project directory in your terminal.

## Usage

The program is executed via the command line with the following syntax:

```
cargo run <pattern> <replacement> <input_file> <output_file>
```

<pattern>: The regular expression pattern to search for in the input file.
<replacement>: The string to replace each occurrence of the pattern with.
<input_file>: The path to the file whose content will be searched.
<output_file>: The path where the modified content will be written.
Example:
```
cargo run "foo" "bar" "./input.txt" "./output.txt"
```
This command will replace all occurrences of "foo" with "bar" in input.txt and save the result to output.txt.
