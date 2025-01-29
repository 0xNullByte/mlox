# Mlox Programming Language Project 🎇
This project is inspired by the book [Crafting Interpreters](https://craftinginterpreters.com/) by  **Robert Nystrom**. It is my personal journey of learning and applying concepts from the book, focusing on building a functional interpreter step-by-step.

# Progress Overview
✅ **Chapter 1: Introduction**\
✅ **Chapter 2: A Map of the Territory** \
✅ **Chapter 3: The Lox Language** \
✅ **Chapter 4: Scanning** \
✅ **Chapter 5: Representing Code** \
✅ **Chapter 6: Parsing Expressions** \
✅ **Chapter 7: Evaluating Expressions** \
✅ **Chapter 8: Statements and State** \
🔄 **Chapter 9: Control Flow** \
🚧 Future chapters: Upcoming plans inshallah

# Mlox Grammar:
**Literals:** `Numbers`, `Strings`, `Booleans` and `null`. \
**Unary expressions:** `!` for not, and `-` to negative. \
**Binary expressions:*** (`+`, `-`, `*`, `/`) and (`==`, `!=`, `<`, `<=`, `>,` `>=`). \
**Parentheses:** `(` and `)`. 
```
program        → declaration* EOF ;

declaration    → varDecl
               | statement ;

statement      → exprStmt
               | printStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

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

# Mlox's Built-in Types:
```
        Mlox         |    Rust representation
----------------------------------------------------
        Obj          |            <T>
        null         |            None
        bool         |            bool
        Num          |            f64
        Str          |            String
```

