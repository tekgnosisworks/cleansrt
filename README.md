# CleanSRT

A fast, command-line tool for removing unwanted text from SRT subtitle files. Designed for batch processing large collections of subtitle files with consistent filtering patterns.

## Features

- ✅ Process single files or entire directories recursively
- ✅ Remove specific text patterns from subtitle entries
- ✅ Automatic subtitle renumbering after text removal
- ✅ Progress bars for large batch operations
- ✅ Backup originals with `.OLD.` suffix
- ✅ Optional cleanup of backup files
- ✅ Detailed logging to `cleansrt.log`

## Installation

### From Source
```bash
git clone https://github.com/yourusername/cleansrt.git
cd cleansrt
cargo build --release
```

The binary will be available at `target/release/cleansrt`

### Requirements
- Rust 1.70+ 
- Cargo

## Usage

### Basic Syntax
```bash
cleansrt -f <INPUT> (-t <TEXT> | -T <TEXT_FILE>) [OPTIONS]
```

### Arguments

| Argument | Short | Long | Description |
|----------|-------|------|-------------|
| **Input** | `-f` | `--file` | SRT file or directory to process (required) |
| **Output** | `-o` | `--output` | Output file (single files only, optional) |
| **Delete** | `-d` | `--delete` | Delete backup `.OLD.` files after processing |
| **Text Filter** | `-t` | `--text` | Single text string to remove (use `\n` for line breaks) |
| **Text File** | `-T` | `--text-file` | File containing text patterns to remove (one per line) |

> **Note:** You must specify either `-t` or `-T`, but not both.

### Examples

**Remove single text pattern from one file:**
```bash
cleansrt -f movie.srt -t "Created by\nSome Subtitle Team"
```

**Process entire directory using text file:**
```bash
cleansrt -f /path/to/subtitles/ -T filters.txt -d
```

**Process with custom output location:**
```bash
cleansrt -f input.srt -o cleaned.srt -t "Advertisement text"
```

### Text Filter File Format

Create a text file with one filter pattern per line. Use `\n` for multi-line patterns:

```
Created by\nSome Subtitle Company
Advertisement
Please rate this subtitle
```

## How It Works

### Single File Processing
1. Creates backup: `movie.srt` → `movie.OLD.srt`
2. Processes original file, removing matching text
3. Renumbers remaining subtitles sequentially
4. Saves cleaned version as `movie.srt`

### Directory Processing
1. Recursively scans for all `.srt` files
2. Shows progress bar during processing
3. Processes each file using the single file workflow
4. Optionally removes backup files if `-d` flag is used

### Output Structure
```
Before:
├── movie.srt

After (without -d):
├── movie.srt         (cleaned)
├── movie.OLD.srt     (original backup)

After (with -d):
├── movie.srt         (cleaned)
```

## Logging

All operations are logged to `cleansrt.log` in the same directory as the executable:

```
2024-12-17 14:30:15 [INFO] - Starting at /path/to/subtitles
2024-12-17 14:30:15 [INFO] - Remove from srt: ["Banana's Subtitle", "Woodchucks Chucking Wood"]
2024-12-17 14:30:16 [INFO] - Found 150 .srt files to process
2024-12-17 14:30:45 [INFO] - Processed: "movie.OLD.srt" -> "movie.srt"
```

## Common Use Cases

**Cleaning downloaded subtitles:** Remove watermarks and advertisement text from subtitle files downloaded from various sources.

**Batch processing:** Clean hundreds or thousands of subtitle files at once with consistent filtering rules.

**Subtitle preparation:** Prepare subtitle files for distribution by removing creator credits or website advertisements.

## Error Handling

- Invalid files are skipped with error logging
- Failed file operations continue processing remaining files
- Progress bars show current status even when errors occur
- All errors are logged with timestamps for troubleshooting

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support Development

If you find this tool useful, consider supporting the development:

- **Zelle:** bmoore@tekgnosis.works  
- **Venmo:** @tekgnosis

Your support helps maintain and improve this project. Thank you!