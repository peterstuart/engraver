use smufl::StaffSpaces;
use svg::node::element::SVG;

use crate::render::ir::{Coord, Element, Line, Linecap, Rect, Symbol, Text};

const SYMBOL_CLASS_NAME: &str = "symbol";
const TEXT_CLASS_NAME: &str = "text";

#[derive(Debug)]
pub struct Options {
    pub symbol_font_name: String,
    pub text_font_family: Vec<String>,
    pub staff_space_to_pixel_ratio: f64,
}

pub fn elements_to_svg_document(
    elements: Vec<Element<StaffSpaces>>,
    options: &Options,
) -> svg::Document {
    let width = elements
        .iter()
        .map(|element| element.max_x())
        .max_by(|x1, x2| x1.partial_cmp(x2).expect("StaffSpaces must be orderable"))
        .unwrap_or(StaffSpaces::zero());
    let height = StaffSpaces(10.0);

    let document = svg::Document::new()
        .set("width", width.0 * 10.0)
        .set("height", height.0 * 10.0)
        .set("viewBox", (0, 0, width.0, height.0 / 2.0))
        .add(style_element(options));

    elements
        .into_iter()
        .map(|element: Element<StaffSpaces>| element.convert(&|staff_spaces| staff_spaces.0))
        .fold(document, |document, element| {
            add_element_to_document(element, document)
        })
}

fn add_element_to_document(element: Element<f64>, document: SVG) -> SVG {
    match element {
        Element::Line(line) => document.add::<svg::node::element::Line>(line.into()),
        Element::Rect(rect) => document.add::<svg::node::element::Rectangle>(rect.into()),
        Element::Symbol(symbol) => document.add::<svg::node::element::Text>(symbol.into()),
        Element::Text(text) => document.add::<svg::node::element::Text>(text.into()),
    }
}

fn style_element(options: &Options) -> svg::node::element::Style {
    let symbol_font_family = &options.symbol_font_name;
    let text_font_family = &options.text_font_family.join(", ");

    let classes = format!(
        r#"
svg {{
  transform: scaleY(-1);
  font-size: 4px;
}}
text {{
  transform: scaleY(-1);
}}
.{SYMBOL_CLASS_NAME} {{
  font-family: "{symbol_font_family}";
  font-size: 1em;
}}
.{TEXT_CLASS_NAME} {{
  font-family: "{text_font_family}";
  font-size: 1px;
}}
"#
    );

    svg::node::element::Style::new(classes)
}

impl From<Line<f64>> for svg::node::element::Line {
    fn from(line: Line<f64>) -> Self {
        let cap = match line.cap {
            Linecap::Butt => "butt",
            Linecap::Round => "round",
        };

        svg::node::element::Line::new()
            .set("x1", line.from.x)
            .set("y1", line.from.y)
            .set("x2", line.to.x)
            .set("y2", line.to.y)
            .set("stroke-width", line.thickness)
            .set("stroke-linecap", cap)
            .set("stroke", "black")
    }
}

impl From<Rect<f64>> for svg::node::element::Rectangle {
    fn from(rect: Rect<f64>) -> Self {
        svg::node::element::Rectangle::new()
            .set("x", rect.origin.x)
            .set("y", rect.origin.y)
            .set("width", rect.size.width)
            .set("height", rect.size.height)
    }
}

fn transform_origin_for_coord(coord: Coord<f64>) -> String {
    format!("{}px {}px", coord.x, coord.y)
}

impl From<Symbol<f64>> for svg::node::element::Text {
    fn from(symbol: Symbol<f64>) -> Self {
        svg::node::element::Text::new()
            .set("x", symbol.origin.x)
            .set("y", symbol.origin.y)
            .set("class", SYMBOL_CLASS_NAME)
            .set(
                "style",
                format!(
                    "transform-origin: {}",
                    transform_origin_for_coord(symbol.origin)
                ),
            )
            .add(svg::node::Text::new(symbol.value))
    }
}

impl From<Text<f64>> for svg::node::element::Text {
    fn from(text: Text<f64>) -> Self {
        svg::node::element::Text::new()
            .set("x", text.origin.x)
            .set("y", text.origin.y)
            .set("class", TEXT_CLASS_NAME)
            .set(
                "style",
                format!(
                    "transform-origin: {}",
                    transform_origin_for_coord(text.origin)
                ),
            )
            .add(svg::node::Text::new(text.value))
    }
}
