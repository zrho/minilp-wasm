A minimal JavaScript binding to the Rust
[minilp](https://github.com/ztlpn/minilp/) library for solving linear programs.

**Warning:** The API is currently undocumented and will likely change.

# Example

```
const minilp = require("@zrho/minilp-wasm");

console.log(minilp.solve({
  direction: "maximize",
  variables: [
    { coefficient: 1, minimum: 0, maximum: 10 },
    { coefficient: 2, minimum: 0, maximum: 10 },
  ],
  constraints: [
    { 
      expression: [
        { variable: 0, coefficient: 1 },
        { variable: 1, coefficient: 1 },
      ],
      comparison: "le",
      constant: 10,
    }
  ]
}));
```
