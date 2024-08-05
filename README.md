# luau-rs

Rust to Luau AST. This is a made-up AST, which can be used for optimization, preprocessing or rendering to code.

This project is in a very early stage, use in production is not recommended.

Main use is inside of [roblox-rs](https://github.com/glomdom/roblox-rs).

## roadmap

In no particular order:
- [x] functions
- [x] returning
  - [x] explicit
  - [x] implicit
- [ ] control flow
  - [x] if
  - [x] else
  - [x] else if
  - [ ] while
  - [ ] for
  - [ ] loop
- [x] function params
  - [x] map most rust primitives to luau
  - [x] deref
  - [x] ref
  - [ ] generics
  - [ ] struct types?
- [ ] structs
  - [ ] impl
- [ ] traits