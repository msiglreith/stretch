#[cfg(test)]
mod root_constraints {
    use stretch::number::*;

    #[test]
    fn root_with_percentage_size() {
        let mut ui = stretch::node::Stretch::new();
        let node = ui.create_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(1.0),
                    height: stretch::style::Dimension::Percent(1.0),
                },
                ..Default::default()
            },
            vec![],
        );
        ui.compute_layout(
            node,
            stretch::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(200.0) },
        )
        .unwrap();

        let layout = ui.layout(node);
        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 200.0);
    }

    #[test]
    fn root_with_no_size() {
        let mut ui = stretch::node::Stretch::new();
        let node = ui.create_node(stretch::style::Style { ..Default::default() }, vec![]);
        ui.compute_layout(
            node,
            stretch::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
        )
        .unwrap();

        let layout = ui.layout(node);
        assert_eq!(layout.size.width, 0.0);
        assert_eq!(layout.size.height, 0.0);
    }

    #[test]
    fn root_with_larger_size() {
        let mut ui = stretch::node::Stretch::new();
        let node = ui.create_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200.0),
                    height: stretch::style::Dimension::Points(200.0),
                },
                ..Default::default()
            },
            vec![],
        );

        ui.compute_layout(
            node,
            stretch::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
        )
        .unwrap();

        let layout = ui.layout(node);
        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 200.0);
    }
}
