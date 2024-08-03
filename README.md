# roblox-rs

Rust to Luau compiler. Heavy inspiration from [roblox-ts](https://roblox-ts.com/)

Current state is mapping rust AST to a custom IR which will ease Luau rendering.

:warning: **DISCLAIMER** -> This crate will be renamed in the near future.

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