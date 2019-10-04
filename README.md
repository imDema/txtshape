# txtshape

Draw the shape of an ascii art with your text or code

## Usage

```
txtshape 
Create ascii text art

USAGE:
    asciicode --ascii <ascii> --text <text> [output]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --ascii <ascii>    Ascii art (be sure whitespace is used where nothing should be written)
    -t, --text <text>      Text content

ARGS:
    <output>     [default: asciicode.out]
```

The `--text` is the file containing the content which you want to be used to draw the shape in `--ascii`

The ascii art is evaluated only with two levels, either something (any valid character) or nothing (whitespace). So ascii art with more shades which isn't whitespace where the shape ends won't work at the moment.

> In the future I may decide to integrate an ascii art generator to use to generate the shape from an image


### Notes

Learning project