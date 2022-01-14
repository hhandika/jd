# jd

![CI](https://github.com/hhandika/jd/workflows/CI/badge.svg)

A command line application to proxify journal url/doi. Current version only works with LSU proxy.

## Installation

To be able to run it in any directory, copy the executable to your path environment. By default the program use LSU Ezproxy. To use a different proxy, create a text file that contain your costum proxy and save it as `ezproxy.txt` (all in lowercases). Put the file in the same directory as your executable.

## Usage

```Bash
jd -i [input-url]
```

Using non-default browser:

```Bash
jd -i [input-url] -b firefox
```
