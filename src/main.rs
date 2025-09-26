// Drawing the (mod draw)
trait GraphRenderer {
    fn rectangle(text: &str);
}

// Layout computation to find a nice layout (mod layout)
enum ShapeKind {
    Diamond,
    Rect,
}

// maybe helpful to be able to compute distance between
// point and nearest point on shape (shape distance)
struct Shape {
    entity_index: usize,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    kind: ShapeKind,
}

struct Edge {
    points: Vec<(f32, f32)>,
    from_shape: usize,
    to_shape: usize,
    is_arrow: bool,
    is_thick: bool,
}

/// Task: ERDiagram will fill with relevant info about graph structure.
/// we are responsible for finding x,y values for shapes and points for
/// edges that make the diagram look nice. Looks nice is quantified as
/// follows:
/// - dist(A, B) < dist(A, C) ∀ A,B,C ∈ 𝕍 where {A,B} exists
///   but {A,C} does not.
/// - Crossovers are minimal and for planar graphs not present.
/// - A ∩ B = {} ∀ A,B ∈ 𝕍.
///
/// BTW be aware that we don't actually have a good way to
struct Layout {
    // These are stored in the same order as entities on `ERDiagram`
    shapes: Vec<Shape>,
    // These aren't in same order as relation cause relations have 2-3
    // edges.
    edges: Vec<Edge>,
    // If this isn't a planar graph we do minimization of edge crossover
    // rather than continue forever.
    is_planar: bool,
}

// Pure representation of diagrams parsed from source code (mod parse)
struct ERDiagram<'source> {
    entities: Vec<Entity<'source>>,
    relations: Vec<Relation<'source>>,
}

enum AttributeKind {
    Key,
    Set,
    Default,
}

struct Entity<'source> {
    name: &'source str,
    attribute: Vec<&'source str>,
    attribute_kind: Vec<AttributeKind>,
}

struct Relation<'source> {
    entity: Entity<'source>,
    relates: Vec<&'source str>,
}

fn main() {}
