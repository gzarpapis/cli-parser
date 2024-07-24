# cli-parser

## Introduction
This library provides a very light-weight API, for a singular purpose: Collect the command line arguments in 3 `std` structures based on the number of dashes prefixed.

- Vector $\leftarrow$ 0 dashes $\leftarrow$ Positional arguments (values only).
- HashSet $\leftarrow$ 1 dash $\leftarrow$ Flags (keys only).
- HashMap $\leftarrow$ 2 dashes $\leftarrow$ Key - value pairs.


## Syntax example:

```bash
./my_program --debug_level=2 -verb path/to/file 
```

These arguments are classified as:
- Positional:
  1. `./my_program`
  2. `path/to/file`
- Flags:
  1. `verb`
- Pairs:
  1. `debug_level` with value `2`

## Usage

Just initialize the CLIParser struct and you are good to go.

```rust
use cliparser::CLIParser;


fn main() {

	// Initialize parser
	let parser = CLIParser::new().init().unwrap();
	
	// Extract parsed data structures
	let posit_arguments = parser.posits.clone(); // Vector
	let flags = parser.flags.clone(); // HashSet
	let pairs = parser.pairs.clone(); // HashMap
}
```