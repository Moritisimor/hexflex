# HexFlex
A hex editor written in Rust

## What is this project about?
HexFlex is a CLI-Editor, but not for text. Instead, it reads and manipulates bytes.

Using it feels very similar to one of my older projects, [Neo-Ed](https://github.com/Moritisimor/Neo-Ed).

## Contributing
Contributions are always more than welcome.

Whether these are bug-fixes or cleaning up code, I am always more than grateful.

## How do I compile it?

### What do I need?
You will only need `Cargo`.

### Cloning and Building 
```bash
git clone https://github.com/Moritisimor/hexflex
cd hexflex
cargo build -r
```

The compiled binary can be found here: `target/release/hexflex`.

## How do I use it?
There are 3 main modes:
- Dumping to a file
- Reverse
- REPL

### Dumping to a file
This mode will simply write the output of hexflex to a file

To use this mode, you will simply need to set the `-o` flag.

Example:
```bash
hexflex input_file -o output_file
```

This will dump the contents of `input_file` into `output_file` in a human-readable format.

Keep in mind that `output_file` might be rather large, depending on how large `input_file` is.

### Reverse
This mode will take the path to a file that contains one or more bytes written as hex, binary or decimal.

It will parse these representations of bytes to actual bytes and write them to ```output_file```.

Setting the ```--reverse``` flag requires the ```--output-file``` flag to be set as well.

Example:
```bash
hexflex input_file -r -o output_file
```

```input_file``` could, for example look like this:
```
0x68 0x65
0x6c
0x6c 0x6f
0x20 0x77
0x6f 0x72
0x6c 0x64
```

```output_file``` would then look like this:
```
hello world
```

### REPL
This mode works very similarly to a shell, meaning that to read and modify the bytes you will need to type some commands.

To use this mode, you will simply need to omit the `-o` flag.

Example:
```bash
hexflex input_file
```

You should then be greeted by a prompt like this:

```bash
[input_file] HexFlex >>
```

From here you can run various commands.

Note that when a command expects a number as an argument, you can either type it as decimal, hex or binary.

Also note that hex numbers require a `0x` prefix, and binary numbers require a `0b` prefix.

#### Read
To read the entire file at once, omit all arguments.

Depending on the size of the file, this may take a while though.

```bash
[input_file] HexFlex >> r
```

To only read a single byte, you simply supply the offset as an argument.

```bash
[input_file] HexFlex >> r 0xff
0x000000ff:     0b01111001      |       0x79    |       121     | (y)
```

You can also read a range of bytes. To do this, you simply supply the start and the end as arguments.

```bash
[input_file] HexFlex >> r 0xa 0xf
0x0000000a:     0b01010100      |       0x54    |       84      | (T)
0x0000000b:     0b01000011      |       0x43    |       67      | (C)
0x0000000c:     0b01001111      |       0x4f    |       79      | (O)
0x0000000d:     0b01001110      |       0x4e    |       78      | (N)
0x0000000e:     0b00000000      |       0x0     |       0
0x0000000f:     0b00000000      |       0x0     |       0
```

#### Edit
```bash
[input_file] HexFlex >> e 0xa
Edit 0x0000000a > 0x54
```

From here you can edit the number to be anything you want, as long as it's between 0 and 255.

#### Nullify
```bash
[input_file] HexFlex >> n 0xa
Successfully nullified 0x0000000a
```

This simply sets the byte to `0x0`.

#### Find String
```bash
[input_file] HexFlex >> fs Hello
Searching...
Match found at: 0x00002004 - 0x00002008
1 Match!
```

This command will search for a given byte sequence.

It will treat the input as a list of characters, casting them to bytes.

#### Find Bytes
```bash
[input_file] HexFlex >> fb 0x48 0x65 0x6c 0x6c 0x6f
Searching...
Match found at: 0x00002004 - 0x00002008
1 Match!
```

This command will also search for a given byte sequence.

It will treat the input as a list of bytes, trying to convert each entry to a byte.

#### Save
You can simply save the buffer to the original file by omitting all arguments.

```bash
[input_file] HexFlex >> s
```

Alternatively, you can also save the buffer to another file. The file will be created if it doesn't exist.

```bash
[input_file] HexFlex >> s input_file_modified
```

#### Clear
```bash
[input_file] HexFlex >> clear
```

This simply clears the screen.

#### Exit
```bash
[input_file] HexFlex >> exit
```

Closes HexFlex.
