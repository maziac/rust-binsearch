# binsearch

A binary search command line tool.

# Arguments

- "**--help**": Prints the help.
- "**--offs** offset": Offset from start of file. Moves last position.
- "**--reloffs** offset": Offset from last position. Moves last position.
- "**--size** size": The number of bytes to evaluate. Moves last position.
- "**--search** string": Searches for the first occurrence of the string. The search starts at last position.
- "**--format** format": The output format:
	- "bin": Binary output. The default.
	- "text":	Textual output. Showing the offset and values in rows.

Examples:
- "binsearch --offs 10 --size 100": Outputs the bytes from position 10 to 109.
- "binsearch --offs 10 --size 100 --offs 200 --size 10": Outputs the bytes from position 10 to 109, directly followed by 200 to 209.
- "binsearch --offs 10 --size 100 --reloffs 10 --size 20": Outputs the bytes from position 10 to 109, directly followed by 120 to 129.
- "binsearch --search 'abc' --size 10": Outputs 10 bytes from the first occurence of 'abc'. If not found nothing is output.


# Developemnt

# Run

~~~
cargo run
~~~
