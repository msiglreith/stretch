use criterion::{criterion_group, criterion_main, Criterion};

fn stretch_benchmarks(c: &mut Criterion) {
    c.bench_function("deep hierarchy", |b| {
        b.iter(|| {
            let mut ui = stretch::node::Stretch::new();

            let node111 = ui.create_node(stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![]);
            let node112 = ui.create_node(stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![]);

            let node121 = ui.create_node(stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![]);
            let node122 = ui.create_node(stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![]);

            let node11 = ui.create_node(stretch::style::Style { ..Default::default() }, vec![node111, node112]);
            let node12 = ui.create_node(stretch::style::Style { ..Default::default() }, vec![node121, node122]);
            let node1 = ui.create_node(stretch::style::Style { ..Default::default() }, vec![node11, node12]);

            let node211 = ui.create_node(stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![]);
            let node212 = ui.create_node(stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![]);

            let node221 = ui.create_node(stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![]);
            let node222 = ui.create_node(stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![]);

            let node21 = ui.create_node(stretch::style::Style { ..Default::default() }, vec![node211, node212]);
            let node22 = ui.create_node(stretch::style::Style { ..Default::default() }, vec![node221, node222]);

            let node2 = ui.create_node(stretch::style::Style { ..Default::default() }, vec![node21, node22]);
            let node0 = ui.create_node(stretch::style::Style { ..Default::default() }, vec![node1, node2]);
            ui
            .compute_layout(node0, stretch::geometry::Size::undefined())
            .unwrap()
        })
    });
}

criterion_group!(benches, stretch_benchmarks);
criterion_main!(benches);
