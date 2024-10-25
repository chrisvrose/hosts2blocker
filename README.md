
# hosts2blocker

Converts a hosts file into a list of domains.

```
Usage: hosts2blocker [OPTIONS]

Options:
  -i, --input-file <INPUT_FILE>    [default: input.txt]
  -o, --output-file <OUTPUT_FILE>  [default: output.txt]
  -h, --help                       Print help
  -V, --version                    Print version
```

#### Example

Input: 
```hosts
0.0.0.0 google.com
0.0.0.0 github.com
```

Output:
```csv
google.com
github.com
```

