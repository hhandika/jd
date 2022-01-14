# ezp

![CI](https://github.com/hhandika/ezp/workflows/CI/badge.svg)

A command line application to proxify journal url/doi.

## Installation

To be able to run it in any directory, download the latest version in [the release page](https://github.com/hhandika/ezp/releases), extract, and copy the executable to your path environment. If have the Rust Package Manager installed in your system, you can install using cargo:

```Bash
cargo install ezp
```

## Using Custom Proxy

By default the program use LSU EZProxy. To use a different proxy:

```Bash
ezp --set [EZProxy-url]
```

You can find EZProxy url in in Paperpile database [here](https://paperpile.com/guides/proxy-list/). For example to set to University of Melbourne EZProxy:

```Bash
ezp --set "http://ezp.lib.unimelb.edu.au/login?url="
```

You only need to do this once. If you would like to change to a different proxy, you can run `--set` again. It will change the proxy setting to the new one.

To reset to the default proxy:

```Bash
ezp --reset
```

## Usage

```Bash
ezp -i [journal-link/doi]
```

It will open your default browser to download the paper using your library access.

For example to download Ou et al. 2022 paper using a doi address:

```Bash
ezp -i https://doi.org/10.1093/biosci/biab133git 
```

Using the web address:

```Bash
ezp -i https://academic.oup.com/bioscience/advance-article/doi/10.1093/biosci/biab133/6482999
```

Using non-default browser:

```Bash
ezp -i [input-url] -b firefox
```

> :warning: For Windows users, the non-default browser only works if you install the app in your native operating system. If you run the app on Windows WSL, it will only work using the default browser.
