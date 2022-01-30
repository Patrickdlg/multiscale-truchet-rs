# multiscale-truchet

A cli for creating multi-scale truchet SVGs - based on https://christophercarlson.com/portfolio/multi-scale-truchet-patterns/

## Usage
```
USAGE:
    multiscale-truchet [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    Output file name [default: output.svg]
    -V, --version            Print version information
    -w, --width <WIDTH>      Width of the output svg in small tiles. Must be greater than 4
                             [default: 16]

```

## Limitations

This is a first-draft that I threw together for fun. The code is messy and it has the following limitations:

- Only supports creation of square SVGs
- Only supports 3 levels of scale
- SVGs are black and white
- SVGs are randomly tiled

Feel free to contribute if you want any of these changed.

## Contributing
Please see CONTRIBUTING.md

## LICENSE / WARRANTY
Please see LICENSE