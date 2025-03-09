# CHANGELOG

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