use stretch::geometry::Size;
use stretch::style::*;

fn main() {
    let mut stretch = stretch::node::Stretch::new();
    let node1 = stretch.create_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        vec![],
    );
    let node0 = stretch.create_node(
        Style {
            size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        vec![node1],
    );
    let layout = stretch.compute_layout(node0, Size::undefined()).unwrap();

    println!("{:#?}", layout);
    println!("{:#?}", stretch.layout(node0));
    println!("{:#?}", stretch.layout(node1));
}
