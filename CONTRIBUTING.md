# Contributing

Thanks for contributing!

## Reporting bugs

Please be as thorough as possible in your reports, ideally isolating it to one lump/part that is failing to parse correctly. 

We'll likely need the map file you're trying to parse, or at least a part of it, so if that's a problem please try to make a new file that shows the same problem.
`transfer.sh` is a good file host.

## Code

If you'd like to contribute code, check the issues tab for others who need your help or, if you have your own idea, open an issue there.

Code should match the code style, be thoroughly tested and documented. Read the notes below about testing and use `clippy` to help with this.

## Testing

You'll need to run `scripts/pre_test.sh` to compile the binary files used for tests.

### .hex

`.hex` files are normal hexdumps that allow single line comments with `//` and ignore spaces/tabs/newlines.
During testing, you can compile these with `scripts/compile_hex.js path/to/file.hex`, the output will be written as a raw binary to `path/to/file.bin`.
If you're adding a test, please add the appropriate line to `scripts/pre_test.sh`.
If you need to modify a `.hex` file, you'll need to recompile it before you re-run your tests.