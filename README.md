# dia
A handcrafted interpreter for a dynamically typed programming language  written in Rust. 
Includes a tokenizer, parser, AST builder, and bytecode-style tree-walk interpreter inspired by Crafting Interpreters.

-----------------------------------------------------
LANGUAGE CORE
-----------------------------------------------------
- Dynamically typed (no type annotations required) 
- Supports:
    • Numbers
    • Strings
    • Booleans
    • nil (null equivalent)
- Arithmetic and logical expressions: + - * / > < == !=
- String concatenation: "Hello " + "World"
- Grouping with parentheses: (a + b * c)
- Variable declaration and assignment: var name = "Mia";
- Block scopes: { ... }
- Lexical scoping rules



✅ DONE
-----------------------------------------------------
- Lexer / Scanner
    • Single-character tokens: () {} , . ; + - * /
    • Keywords: and, class, else, false, for, fun, if, nil, or, print, return, super, this, true, var, while
    • Identifiers and variable names
    • Numbers (integers)
    • Whitespace and line tracking
    • Basic error reporting for unexpected characters
- Parser (basic structure ready)
    • Expression and statement skeletons
- Interpreter (basic runtime)
    • Tree-walk evaluation skeleton
    • Token and AST structures
- Parser
    • Expression parsing (binary, unary, grouping)
    • Statement parsing (expression, print, block)
    • Control flow: if / else, while, for loops

🔜 TODO (Next Features)
-----------------------------------------------------
- Lexer / Scanner
    • Multi-character operators: == != <= >=
    • String literals: "Hello Mia"
    • Comments: // single-line, /* multi-line */
- Interpreter
    • Variable environments and scoping
    • Dynamic typing
    • Boolean and nil evaluation
    • Print statement execution
    • Error handling with line numbers

🕐 LATER (Advanced Features)
-----------------------------------------------------
- Functions
    • Declaration and invocation
    • Parameters and return values
    • Closures and nested functions
- Classes & Objects
    • Class definitions, instances, methods
    • Inheritance and super
    • this keyword
- Built-in functions
    • clock(), len(), type()
- REPL (interactive shell)
- AST visualizer / debug printing

💡 FUTURE / OPTIONAL
-----------------------------------------------------
- Compile to bytecode (VM backend)
- Garbage collection
- Standard library modules (math, string, file I/O)
- REPL enhancements: syntax highlighting, history
- Native extensions / FFI

=====================================================
*/


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
```
