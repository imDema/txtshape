# txtshape

Format text or code in the shape of ascii art

## Usage

```
Shape text into ascii art

USAGE:
    txtshape --shape <shape> --text <text> [output]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --shape <shape>    The shape you want to fit the text in
    -t, --text <text>      Text content

ARGS:
    <output>     [default: txtshape.out]
```

The `--text` is the file containing the content which you want to be used to draw the shape in `--shape`

The ascii art is evaluated only with two levels, either something (any non-whitespace character) or nothing (whitespace). So ascii art that uses different textual characters to represent shades instead of having a defined whitespace / non-whitespace shape won't work at the moment.

> In the future I may decide to integrate an ascii art generator to use to generate the shape from an image


### Notes

Learning project
