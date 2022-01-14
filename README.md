# ezp

![CI](https://github.com/hhandika/ezp/workflows/CI/badge.svg)

A command line application to proxify journal url/doi. Current version only works with LSU proxy.

## Installation

To be able to run it in any directory, copy the executable to your path environment. By default the program use LSU EZProxy. To use a different proxy:

```Bash
ezp --set [EZProxy-url]
```

For example to set to University of Melbourne EZProxy:

```Bash
ezp --set "http://ezp.lib.unimelb.edu.au/login?url="
```

You only need to do this once. You can find EZProxy url in in Paperpile database [here](https://paperpile.com/guides/proxy-list/)

## Usage

```Bash
ezp -i [input-url]
```

Using non-default browser:

```Bash
ezp -i [input-url] -b firefox
```
