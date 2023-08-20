# binsearch

A binary search command line tool.
The tool allows you to specify one (or more files) and dump out values to stdout from a specific offset and for a specific size.
It is also possible to search for patterns to set the offset.

Note: Please also have a look [here(https://github.com/maziac/zig-binsearch)] for more options.


# Arguments

- "**--help**": Prints the help.
- "**--offs** offset": Offset from start of file. Moves last position. You can also move relatively by prefixing with + or -.
- "**--size** size": The number of bytes to output. Moves last position.
- "**--search** string": Searches for the first occurrence of the string. The search starts at last position.

Examples:
- "binsearch --offs 10 --size 100": Outputs the bytes from position 10 to 109.
- "binsearch --offs 10 --size 100 --offs 200 --size 10": Outputs the bytes from position 10 to 109, directly followed by 200 to 209.
- "binsearch --offs 10 --size 100 --reloffs 10 --size 20": Outputs the bytes from position 10 to 109, directly followed by 120 to 129.
- "binsearch --search 'abc' --size 10": Outputs 10 bytes from the first occurrence of 'abc'. If not found nothing is output.


# Development

# Run

~~~
cargo run
~~~
