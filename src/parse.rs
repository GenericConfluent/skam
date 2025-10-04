use glam::Vec2;

// Pure representation of diagrams parsed from source code (mod parse)
#[derive(Default)]
pub struct ERDiagram<'source> {
    entities: Vec<Entity<'source>>,
    relations: Vec<Relation<'source>>,
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
