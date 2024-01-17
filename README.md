## Rust Implementation of Bitcoin 
---
Following along "Programming Bitcoin" O'Reilly book. Building an implementation of a cryptocurrency in Rust.

Noteworthy points:
* `src/field_element/mod.rs` in `impl Div for FieldElement` I do some cool magic to get around overflowing multiply