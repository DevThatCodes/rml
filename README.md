Rml is a version of Xml that i made in rust because i thought it would be a fun project to work on
# Current Features
 - a way to "express/write" rml in rust with structs
# Examples
## Container
Rml:
```rml
[container id="root"]
  [text] This is in the container#root [!text]
[!container]
```
equivalent with Rust structs:
```rust
RmlElement {
    name: "container".to_string(),
    children: vec![
        RmlElement {
            name: "text".to_string(),
            children: Vec::new(),
            tags: vec![
                RmlTag {
                    name: "*content*".to_string(),
                    value: "this is in the container#root".to_string()
                }
            ]
        }
    ],
    tags: vec![
        RmlTag {
            name: "id".to_string(),
            value: "root".to_string()
        }
    ]
}

```
equivalent in Xml:
```xml
<container id="root">
    <text> This is in the container#root </text>
</container>
```
