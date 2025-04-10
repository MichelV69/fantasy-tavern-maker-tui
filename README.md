
# Fantasy Tavern Maker
***For Fantasy Worlds Settings such as Dungeons & Dragons 5th Edition***

## Current Version
0.8.0-96

[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/)
[![made-with-Markdown](https://img.shields.io/badge/Made%20with-Markdown-1f425f.svg)](http://commonmark.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/MichelV69/fantasy-tavern-maker-tui/graphs/commit-activity)
[![GitHub commits](https://badgen.net/github/commits/MichelV69/fantasy-tavern-maker-tui)](https://GitHub.com/MichelV69/fantasy-tavern-maker-tui/commit/)
[![GitHub latest commit](https://badgen.net/github/last-commit/MichelV69/fantasy-tavern-maker-tui)](https://GitHub.com/MichelV69/fantasy-tavern-maker-tui/commit/)
[![GitHub issues](https://img.shields.io/github/issues/MichelV69/fantasy-tavern-maker-tui.svg)](https://GitHub.com/MichelV69/fantasy-tavern-maker-tui/issues/)

[![Rust](https://github.com/MichelV69/fantasy-tavern-maker-tui/actions/workflows/rust.yml/badge.svg)](https://github.com/MichelV69/fantasy-tavern-maker-tui/actions/workflows/rust.yml)
[![CodeQL](https://github.com/MichelV69/fantasy-tavern-maker-tui/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/MichelV69/fantasy-tavern-maker-tui/actions/workflows/github-code-scanning/codeql)
[![Dependabot Updates](https://github.com/MichelV69/fantasy-tavern-maker-tui/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/MichelV69/fantasy-tavern-maker-tui/actions/workflows/dependabot/dependabot-updates)

## Purpose
Most Games Masters or Authors are aware that they need to unique and
interesting Pubs, Taverns, Inns, Wayhouses and the like as part of the
adventure-travel narrative. However, it can be awkward to do so easily
on-the-fly without complex table rolling that interrupts the flow of the story.

This application is a RUST re-write of an original C# implementation
that I wrote which I was never happy with. So far, this version is much
more what I was hoping to achieve.

## Sample Output

![screenshot of example TUI output](content/2025-mar-14-example.png)

# user requirements
Ensure that you have a folder called "fantasy-tavern-maker-tui" in your "Documents" folder.
This is where you will have to unzip the provided PSV data tables.

![screenshot of example data folder](content/2025-mar-15-doc-folder.png)

# libraries in use

https://rust-random.github.io/book/quick-start.html -- RAND functions

https://docs.rs/cursive/0.21.1/cursive/ -- cursive
