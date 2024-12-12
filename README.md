# File Sorter

The File Sorter is a Rust program that scans a directory and its subdirectories (if desired) and sorts the files into new directories based on their file types.

## Features

- Recursively scans a directory and its subdirectories
- Sorts files into new directories based on their file type
- Preserves file metadata (creation and modification dates)
- Allows the user to choose whether to scan subdirectories or not

## Usage

1. Build the Rust program:
   ```
   cargo build
   ```
2. Run the program, passing the directory you want to scan as an argument:
   ```
   cargo run /path/to/directory
   ```
   If no directory is provided, the program will use the current directory.
3. When prompted, enter 'y' to scan subdirectories or 'n' to only scan the provided directory.

The program will create new directories in the root directory based on the file types found, and move the files into their respective directories.

## Example

Suppose you have the following directory structure:

```
/path/to/directory/
├── document1.txt
├── image1.jpg
├── document2.txt
└── video1.mp4
```

Running the File Sorter on this directory will create the following structure:

```
/path/to/directory/
├── txt/
│   ├── document1.txt
│   └── document2.txt
├── jpg/
│   └── image1.jpg
└── mp4/
    └── video1.mp4
```

## Dependencies

The File Sorter uses the following Rust crates:

- `std` - for common Rust standard library functionality
- `chrono` - for handling date and time information

## Contributing

Contributions to the File Sorter are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
