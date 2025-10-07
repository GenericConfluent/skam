use std::collections::hash_map::Keys;

use glam::Vec2;

// Pure representation of diagrams parsed from source code (mod parse)
#[derive(Default)]
pub struct ERDiagram<'source> {
    entities: Vec<Entity<'source>>,
    relations: Vec<Relation<'source>>,
}

struct Lexer<'source> {
    text: &'source str,
    buf: String,
    idx: usize,
}

impl Lexer<'_> {
    fn try_consume_structure_token(ch: char) -> Option<TokenKind> {
        match ch {
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            ';' => TokenKind::Comma,
            ',' => TokenKind::Semicolon,
            _ => return None,
        }
        .into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    start: usize,
    end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TokenKind {
    Entity,
    Relate,
    Weak,
    Single,
    Require,
    Isa,
    Cover,
    Overlap,
    Ident,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.text.chars().nth(self.idx)?;

        if let Some(kind) = Lexer::try_consume_structure_token(current) {}

        let kind = match current {
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            ';' => TokenKind::Comma,
            ',' => TokenKind::Semicolon,
            _ => {
                self.buf.clear();
                self.buf.push(current);

                loop {
                    self.idx += 1;
                    let Some(ch) = self.text.chars().nth(self.idx) else {
                        break;
                    };
                    if ch.is_whitespace() {
                        break;
                    }
                }
            }
        };

        Token {
            kind,
            start: self.idx,
            end: {
                self.idx += 1;
                self.idx
            },
        }
        .into()
    }
}

impl<'source> ERDiagram<'source> {
    pub fn include(&mut self, source: &'source str) {
        todo!()
    }

    /// document = entity_stmt | relate_stmt | isa_stmt
    fn document() {}

    /// entity_stmt = "entity" ident (attr_block | ";")
    fn entity_stmt() {}

    /// isa_stmt = "isa" ident ["(" isa_constraint {"," isa_constraint} ")"] ident {"," ident}
    /// isa_constraint = "cover" | "overlap"
    fn isa_stmt() {}

    /// relate_stmt = "weak"? "relate" ident "(" related_entity, related_entity ("," related_entity)? ")" (attr_block | ";")
    /// related_entity = ("require" | "single") ident
    fn relate_stmt() {}

    /// attr_block = "{" attr* "}"
    /// attr = "key"? "set"? ident ["," ident] "\n"
    fn attr_block() {}

    /// ident = ("\"" ? any thing not " ? "\"") | (alphanumeric | "-" | "_")+
    fn ident() {}
}

pub struct Isa<'source> {
    superclass: &'source str,
    subclasses: Vec<&'source str>,
    // default: false => not every superclass is a member of a subclass
    covering: bool,
    // default: false => no entity in multiple subclasses
    overlap: bool,
}

pub enum AttributeKind {
    Key,
    Set,
    Default,
}

pub struct Entity<'source> {
    name: &'source str,
    attribute: Vec<&'source str>,
    attribute_kind: Vec<AttributeKind>,
}

pub enum RelationConstraint {
    Single,
    Require,
}

pub struct Relation<'source> {
    entity: Entity<'source>,
    relates: Vec<(&'source str, RelationConstraint)>,
}
