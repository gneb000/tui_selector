# tui_selector
Text based list selector, reads a list from stdin and prints selected items to stdout.

## Usage
```
tui_selector [OPTIONS]
```

#### Options
```
-n, --numbering    Add line numbers
-i, --id-mode      Provide list with format "ID::line\n", output selected IDs (more details below)
-h, --help         Print help
-V, --version      Print version
```

#### ID Mode
Provided list has an ID for each line and should be used as output, but not displayed in the selector. Use "::" as delimiter between the ID and the line content, ID goes first (i.e., "ID::line_content").