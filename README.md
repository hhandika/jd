# jd

![CI](https://github.com/hhandika/jd/workflows/CI/badge.svg)

A command line application to proxify journal url/doi. Current version only works with LSU proxy.

## Installation

To be able to run it in any directory, copy the executable to your path environment. By default the program use LSU EZProxy. To use a different proxy:

```Bash
jd --set [EZProxy-url]
```

For example to set to University of Melbourne EZProxy:

```Bash
jd --set "http://ezp.lib.unimelb.edu.au/login?url="
```

You can find exproxy url in in Paperpile database [here](https://paperpile.com/guides/proxy-list/)

## Usage

```Bash
jd -i [input-url]
```

Using non-default browser:

```Bash
jd -i [input-url] -b firefox
```
