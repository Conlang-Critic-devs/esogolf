#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Value(Val),
    BeginBlock,
    EndBlock,
    EmptyBlock,
    Pack,
    Size,
    Length,
    Dup,
    Not,
    If,
    Define,
    ApplyFunction,
    Read,
    Swap,
    Split,
    Get,
    DupGet,
    Move,
    Grab,
    DupGrab,
    Include,
    Drop,
    Type,
    ToFloat,
    ToInt,
    ToBool,
    Eq,
    Neq,
    GreaterThan,
    GreaterEquals,
    LessThan,
    LessEquals,
    Write,
    Print,
    Or,
    And,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

use crate::value::Value as Val;
use self::Command::*;

impl Command {
    pub fn from_str(cmd: &str) -> Self {
        match &*cmd.to_lowercase() {
            "{"|"["|"do" => BeginBlock,
            "}"|"]"|"end" => EndBlock,
            "{}"|"[]"|"nop" => EmptyBlock,
            "inc"|"include" => Include,
            "@" | "pack" => Pack,
            "size" => Size,
            "len" => Length,
            ";" | "dup" => Dup,
            "!" | "not" => Not,
            "?" | "if" => If,
            ":=" | "def" => Define,
            "()" | "apply" => ApplyFunction,
            "<-" | "read" => Read,
            "$" | "swap" => Swap,
            "\\/" | "\\\\/" | "split" => Split,
            "." | "get" => Get,
            ";." | "dupget" => DupGet,
            "<>" | "move" => Move,
            "¤" | "grab" => Grab,
            ":" | "dupgrab" => DupGrab,
            "~" | "drop" => Drop,
            "t" | "type" => Type,
            "#" | "float" => ToFloat,
            "i" | "int" => ToInt,
            "b" | "bool" => ToBool,
            "==" | "=" | "eq" => Eq,
            "!=" | "~=" | "neq" => Neq,
            ">" => GreaterThan,
            ">=" => GreaterEquals,
            "<" => LessThan,
            "<=" => LessEquals,
            "->" | "wrte" => Write,
            "_" | "prnt" => Print,
            "|" | "or" => Or,
            "&" | "and" => And,
            "+" | "add" => Add,
            "-" | "sub" => Sub,
            "*" | "mul" => Mul,
            "/" | "div" => Div,
            "%" | "rem" => Rem,
            _ => Value(Val::parse(cmd))
        }
    }
}
