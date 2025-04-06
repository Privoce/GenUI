# CHANGELOG

## Release V0.1.2 (2025-04-06)

### General

- [x] `if_else_if_else` syntactic sugar
  - [x] basic 
  - [x] nested
- [x] router
  - [x] configuration
  - [x] code
    - [x] `nav_to!` macro
    - [x] `nav_back!` macro
    - [x] component inherits
- [x] add lifecycle
  - [x] `#[before_update]` macro
  - [x] `#[update]` macro
- [x] self-close tag for component in `<template></template>`
- [x] Allow single_script strategies to use genui api
- [x] automatic id strategy (see `utils::Ulid`), all components will automatically generate id
- [x] the define page automatically add getter and setter from `GView` component
- [x] add redraw if callback events need
- [x] adjust the getter｜setter so that both the original object and the ref have two-way binding adjustment
  - [x] genui
  - [x] builtin
- [x] use `Result<(), Box<std::error::Error>>` as setter's return value
- [x] replace `use gen_component::*;` to `use gen_components::{themes::*, utils::*, *};`
- [x] add Default trait for Deref Prop struct 
- [x] computed: `#[computed([args...])]` for powerful value bind

### Fixes

- if the regular method uses (self), you need to add cx as a regular parameter

### Tests

See: [v0.1.2 Tests](https://github.com/Privoce/made_with_GenUI/tree/main/tests)

## Release V0.1.1 (2025-03-09)

### Fixes

- Analyzer
  - Value
    - Enum: parse_template can not parse standard string type
    - Bind: `ident()` output wrong bind ident
  - multi err_from_to
  - Util
    - git download function

### General

- Add builtin components
  - popup
  - popup_container
  - dialog
  - drawer
  - tool_tip
  - drop_down
  - tag
  - loading
  - collapse
  - toggle
- Optimize builtin components
  - image: `src` can use `base64`, `local_file`, `url`
  - radio: animation, lifecycle
  - checkbox: animation, lifecycle
- combine `gen_parser` with `gen_converter` as `gen_analyzer`
- `gen_analyzer`
  - remove one convert layer (from 640.625µs(avg) -> 200.132µs)
  - reduce the number of repeated traversals
  - early static analysis
- `rssyin`
  - visitor_chain -> define analyzer use `ra_ap_syntax` 
  - add new `ScriptBridger`
- remove `gen_mk_script_objs`
- new rust script syntax
  - add `#[component]`
  - use `#[prop]` on prop struct
  - replace lifecycle `#[before_create]` to `#[before_mount]`
  - add `#[mounted]`
  - replace `default_prop!` to `impl Default trait` (just like usual rust)
  - impl `#[component]` struct

### Others

See [ract CHANGELOG](https://github.com/Privoce/ract/blob/main/CHANGELOG.md)