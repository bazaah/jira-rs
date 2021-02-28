# Jira Rust

A library for interacting with Jira's API in Rust.

## Status

This library is still in early development, and has a very limited set of features.
It will be filled out over the coming weekends, you can check the currently supported
API endpoints here: [Features](./features.md).

This project is not currently published to crates.io, as is being developed for personal use.
If you are interested in it being published please open an issue.

## Under Development

- [x] libjira:options/unit-tests
- [x] libjira:models/unit-tests
- [x] libjira:models/cow
- [ ] editor:expose API for `$EDITOR` usage
- [ ] mdtoj:expose API for `.md` -> `jira` formatter
- [ ] cli:issues/create using `$EDITOR`
- [ ] libjira:models/documentation

## Prior Work

This project got the initial design from [goji](https://github.com/softprops/goji).

## License

This project is licensed under [Apache 2](./LICENSE-APACHE) or [MIT](./LICENSE-MIT) at your option.

