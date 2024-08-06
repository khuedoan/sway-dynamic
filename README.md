# sway-dynamic

When used on Sway, this adds a dynamic tiling feature similar to [dwm](https://dwm.suckless.org):

> In tiled layout windows are managed in a master and stacking area.
> The master area contains the window which currently needs most attention, whereas the stacking area contains all other windows.

```
+-------------+------------+
|             |     S1     |
|             +------------+
|      M      |     S2     |
|             +------------+
|             |     S3     |
+-------------+------------+
```

## Usage

Simply run the program `sway-dynamic`. To start it automatically, put it in your sway config like this: `exec sway-dynamic`.

For more info run `sway-dynamic --help`.

## Installation

TBD

## Acknowledgments

- Forked from [ammgws/autotiling-rs](https://github.com/ammgws/autotiling-rs), which alternates the container layout between horizontal and vertical for successive new containers
- [dwm](https://dwm.suckless.org)'s dynamic tiling layout
