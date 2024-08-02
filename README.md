# roblox-rs

Rust to Luau compiler. Heavy inspiration from [roblox-ts](https://roblox-ts.com/)

Current state is mapping rust AST to a custom IR which will ease Luau rendering.

## roadmap

In no particular order:
- [x] functions
- [x] returning
  - [x] explicit
  - [x] implicit
- [x] function params
  - [ ] generics
  - [ ] map types to luau types
  - [ ] struct types?
- [ ] structs
  - [ ] impl
- [ ] traits