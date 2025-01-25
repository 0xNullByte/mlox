#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug)]
pub enum Object {
    Str(String),
    Num(f64),
    None,
}
impl Object {
    pub fn to_string(&self) -> String {
        match self {
            Self::None => "None".into(),
            Self::Str(v) => v.into(),
            Self::Num(n) => n.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
    literal: Object,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
