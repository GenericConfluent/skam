use glam::Vec2;

use crate::{layout::Layout, parse::ERDiagram};

// Drawing the graph (mod draw)
pub trait GraphRenderer {
    fn rectangle(text: &str, position: Vec2, size: Vec2);
    fn diamond(text: &str, position: Vec2, size: Vec2);
    fn edge(text: &str, points: &[Vec2]);

    fn render(&mut self, diagram: ERDiagram, layout: Layout) {
        todo!()
    }
}

impl GraphRenderer for svg::Document {
    fn rectangle(text: &str, position: Vec2, size: Vec2) {}

    fn diamond(text: &str, position: Vec2, size: Vec2) {}

    fn edge(text: &str, points: &[Vec2]) {}
}
