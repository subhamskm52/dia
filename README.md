# dia
A handcrafted interpreter for a dynamically typed programming language  written in Rust. Includes a tokenizer, parser, AST builder, and bytecode-style tree-walk interpreter inspired by Crafting Interpreters.



## 🧩 Example Mia Code

```mia
var x = 10;
var y = 20;

if (x < y) {
    print "x is smaller";
} else {
    print "y is smaller";
}

while (x < y) {
    print x;
    x = x + 1;
}
