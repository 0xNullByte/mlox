# Mlox Grammar:
**Literals:** `Numbers`, `Strings`, `Booleans` and `null`. \
**Unary expressions:** `!` for not, and `-` to negative. \
**Binary expressions:*** (`+`, `-`, `*`, `/`) and (`==`, `!=`, `<`, `<=`, `>,` `>=`). \
**Parentheses:** `(` and `)`. 
```
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "null" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
```

# Mlox Precedence Rules:
```
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "null"
               | "(" expression ")" ;
```
