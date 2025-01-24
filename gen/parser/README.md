# Parse Gen File to AST

## Parse Plan

1. parse the outer : `&str to Vec<Targets> to ParseTarget`
2. parse the inner : `ParseTarget to AST`

## parse outer

外层采用 nom 进行直接转换

outer use nom to parse directly

The aim of outer parsing is to divide the entire file into three main(Ⓜ️) parts:

1. template (optional) Ⓜ️
2. script (optional) Ⓜ️
3. style (optional) Ⓜ️
4. comment (optional)

### parse result

#### normal

##### target

```rust
let input = r#"
        //! This is a comment1
        //! This is a comment2
        //! This is a comment3
        <template>
            <window class="ui">
            </window>
        </template>

        // This is line comment
        /// This is a doc comment
        /// hello
        <script>
        let mut counter:usize = 0

        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>

        // This is line comment2
        <style>
        .ui{
            height : fill;
            width : fill;
            show-bg : true;
        }
        </style>
        // end of line comment
        "#;
```

##### result

```rust
[parser/src/ast/mod.rs:310] target = Ok(
    ParseTarget {
        template: Some(
            "<window class=\"ui\">\n            </window>\n        ",
        ),
        script: Some(
            "let mut counter:usize = 0\n\n        let handle_actions:FnOnce()->() = || {\n            counter += 1;\n        }\n        ",
        ),
        style: Some(
            ".ui{\n            height : fill;\n            width : fill;\n            show-bg : true;\n        }\n        ",
        ),
        comment: Some(
            [
                OfflineComment {
                    value: [
                        File(
                            "This is a comment1",
                        ),
                        File(
                            "This is a comment2",
                        ),
                        File(
                            "This is a comment3",
                        ),
                    ],
                    position: AboveTemplate,
                },
                OfflineComment {
                    value: [
                        Normal(
                            "This is line comment",
                        ),
                        Document(
                            "This is a doc comment",
                        ),
                        Document(
                            "hello",
                        ),
                    ],
                    position: AboveScript,
                },
                OfflineComment {
                    value: [
                        Normal(
                            "This is line comment2",
                        ),
                    ],
                    position: AboveStyle,
                },
                OfflineComment {
                    value: [
                        Normal(
                            "end of line comment",
                        ),
                    ],
                    position: End,
                },
            ],
        ),
    },
)
```

#### empty

```rust
[parser/src/ast/mod.rs:318] target = Err(
    ParseError(
        "The current file has no content. It should be removed to ensure your program has clean file tree!",
    ),
)
```

#### only code

```rust
        let input = r#"
        let a:&str = "trest";
        "#;
```

```rust
[parser/src/ast/mod.rs:328] target = Err(
    ParseError(
        "Parsing file exception. The current file contains content that is not covered by processed tags. If it is a rust script, please wrap it in a `<script>` tag",
    ),
)
```

## parse inner

**Block parsing**

1. no `<template>` tag and no `<style>` tag --> parse as rust script (1 thread)
2. no `<template>` tag and no rust script has `<style>` tag --> parse as style (1 thread)
3. no `<style>` tag and no rust script has `<template>` tag --> parse as template (1 thread)
4. has `<template>` tag and rust script no `<style>` tag --> parse as template_script (2 thread)
5. has 3 tag --> parse as whole gen (3 thread)

<img src="./wiki/inner.png">

### parse template

see parse_template.md

### parse_style

see parse_style.md

### parse_script

see parse_script.md
