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

pub struct Relation<'source> {
    entity: Entity<'source>,
    relates: Vec<&'source str>,
}
