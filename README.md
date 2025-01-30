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
✅ **Chapter 9: Control Flow** \
🔄 **Chapter 10: Functions** \
🚧 Future chapters: Upcoming plans inshallah

# Mlox Grammar:
```
program        → declaration* EOF ;

declaration    → varDecl
               | statement ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
statement      → exprStmt
               | ifStmt
               | printStmt
               | whileStmt
               | block ;

exprStmt       → expression ";" ;
ifStmt         → "if" "(" expression ")" statement 
					  ( "else" statement )? ;
printStmt      → "print" expression ";" ;
whileStmt      → "while" "(" expression ")" statement ;
block          → "{" declaration* "}" ;

expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | logic_or ;

logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;

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
        Null         |            None
        Bool         |            bool
        Num          |            f64
        Str          |            String
```

