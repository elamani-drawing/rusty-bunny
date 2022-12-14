## Summary
- [Summary](#summary)
- [Description](#description)
- [Demo](#demo)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)
- [Auteurs](#auteurs)

## Description

This project was born after because I wanted to make the [rusty-bunny](https://developers.facebook.com/blog/post/2020/06/03/build-smart-bookmarking-tool-rust-rocket/) tutorial to practice rust.  

This is a bunnylol clone programmed in rust.

## Demo
![demo of rusty bunny](demo.gif)

## Installation
0. Install [Rust](https://www.rust-lang.org/)
1. Clone rusty-bunny
```sh
    git clone https://github.com/elamani-drawing/rusty-bunny
```
2. Make sure you're using nightly
```sh
    cargo --version
    //else run: rustup default nightly
```
3. Run the projet
```sh
    cargo run
```
5. Visit [localhost:8000](http://localhost:8000/)

6. Use app from link: "http://localhost:8000/search?cmd=" or settings your [client browser](/docs/setting_browser.md)

## Usage

To test out a command, type in http://localhost:8000/search?cmd= followed by your command.

The following commands are supported by `rusty-bunny`:
- "tw" -> redirects to twitter.com
- "tw @username" -> redirects to twitter.com/username
- "gh" -> redirects to github.com
- "gh @username" -> redirects to github.com/username
- "gh @username/repo" -> redirects to github.com/username/repo
- "gh search" -> make a search in github
- ..for more commands see [list commands](/docs/list_command.md)
Everything else redirects to a google search with your query.

## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

## Auteurs

1. [Au = Or[79]](https://github.com/elamani-drawing)