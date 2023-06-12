use smufl::StaffSpaces;

use crate::render::ir::{Convert, Element, Group, Line, Linecap, Polygon, Symbol, Text};

const SYMBOL_CLASS_NAME: &str = "symbol";
const TEXT_CLASS_NAME: &str = "text";

const RATIO: f64 = 10.0;

#[derive(Debug)]
pub struct Options {
    pub symbol_font_name: String,
    pub text_font_family: Vec<String>,
    pub staff_space_to_pixel_ratio: f64,
}

struct Converter {
    ratio: f64,
    staff_origin: StaffSpaces,
}

impl Convert<StaffSpaces, f64> for Converter {
    fn convert_x(&self, x: StaffSpaces) -> f64 {
        x.0 * self.ratio
    }

    fn convert_y(&self, y: StaffSpaces) -> f64 {
        (self.staff_origin - y).0 * self.ratio
    }

    fn convert_thickness(&self, thickness: StaffSpaces) -> f64 {
        thickness.0 * self.ratio
    }
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
        .set("width", width.0 * RATIO)
        .set("height", height.0 * RATIO)
        .add(style_element(options));

    let staff_origin = StaffSpaces(2.5) + StaffSpaces(5.0);

    let converter = Converter {
        ratio: RATIO,
        staff_origin,
    };

    elements
        .into_iter()
        .map(|element: Element<StaffSpaces>| element.convert(&converter))
        .fold(document, |document, element| {
            add_element_to_node(element, document)
        })
}

fn add_element_to_node<N: svg::node::Node>(element: Element<f64>, mut node: N) -> N {
    match element {
        Element::Line(line) => node.append::<svg::node::element::Line>(line.into()),
        Element::Polygon(polygon) => node.append::<svg::node::element::Polygon>(polygon.into()),
        Element::Symbol(symbol) => node.append::<svg::node::element::Text>(symbol.into()),
        Element::Text(text) => node.append::<svg::node::element::Text>(text.into()),
        Element::Group(Group { id, elements }) => {
            let mut group = svg::node::element::Group::new();
            if let Some(id) = id {
                group = group.set("id", id);
            }
            for element in elements {
                group = add_element_to_node(element, group);
            }
            node.append(group)
        }
    }
    node
}

fn style_element(options: &Options) -> svg::node::element::Style {
    let font_size = RATIO * 4.0;
    let symbol_font_family = &options.symbol_font_name;
    let text_font_family = &options.text_font_family.join(", ");

    let classes = format!(
        r#"
svg {{
  font-size: {font_size}px;
  color: black;
}}
.{SYMBOL_CLASS_NAME} {{
  font-family: "{symbol_font_family}";
  font-size: 1em;
}}
.{TEXT_CLASS_NAME} {{
  font-family: "{text_font_family}";
  font-size: 10px;
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

impl From<Polygon<f64>> for svg::node::element::Polygon {
    fn from(polygon: Polygon<f64>) -> Self {
        let points_as_strings: Vec<_> = polygon
            .points()
            .iter()
            .map(|coord| format!("{},{}", coord.x, coord.y))
            .collect();
        let points = points_as_strings.join(", ");

        svg::node::element::Polygon::new().set("points", points)
    }
}

impl From<Symbol<f64>> for svg::node::element::Text {
    fn from(symbol: Symbol<f64>) -> Self {
        svg::node::element::Text::new()
            .set("x", symbol.origin.x)
            .set("y", symbol.origin.y)
            .set("class", SYMBOL_CLASS_NAME)
            .add(svg::node::Text::new(symbol.value))
    }
}

impl From<Text<f64>> for svg::node::element::Text {
    fn from(text: Text<f64>) -> Self {
        svg::node::element::Text::new()
            .set("x", text.origin.x)
            .set("y", text.origin.y)
            .set("class", TEXT_CLASS_NAME)
            .add(svg::node::Text::new(text.value))
    }
}
