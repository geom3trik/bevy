use bevy::prelude::*;
use bevy::ui::{LayoutType, PositionType, Units, TreeBuilder};
/// This example illustrates the various features of Bevy UI.
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    // root node

    commands.spawn_bundle(NodeBundle {
        style: Style {
            layout_type: LayoutType::Grid,
            grid_rows: vec![Units::Stretch(1.0), Units::Pixels(200.0), Units::Stretch(1.0), Units::Stretch(1.0)],
            grid_cols: vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Pixels(200.0)],
            ..Default::default()
        },
        material: materials.add(Color::AZURE.into()),
        ..Default::default()
    }).with_node_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                row_index: 0,
                row_span: 1,
                col_index: 0,
                col_span: 2,
                ..Default::default()
            },
            material: materials.add(Color::OLIVE.into()),
            ..Default::default()
        });

        parent.spawn_bundle(NodeBundle {
            style: Style {
                row_index: 0,
                row_span: 1,
                col_index: 2,
                col_span: 1,
                ..Default::default()
            },
            material: materials.add(Color::VIOLET.into()),
            ..Default::default()
        });

        parent.spawn_bundle(NodeBundle {
            style: Style {
                row_index: 1,
                row_span: 2,
                col_index: 0,
                col_span: 1,
                layout_type: LayoutType::Grid,
                grid_rows: vec![Units::Stretch(1.0), Units::Pixels(75.0), Units::Stretch(1.0), Units::Stretch(1.0)],
                grid_cols: vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Pixels(50.0)],
                ..Default::default()
            },
            material: materials.add(Color::TOMATO.into()),
            ..Default::default()
        }).with_node_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    row_index: 0,
                    row_span: 1,
                    col_index: 0,
                    col_span: 2,
                    ..Default::default()
                },
                material: materials.add(Color::FUCHSIA.into()),
                ..Default::default()
            });
    
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    row_index: 0,
                    row_span: 1,
                    col_index: 2,
                    col_span: 1,
                    ..Default::default()
                },
                material: materials.add(Color::BISQUE.into()),
                ..Default::default()
            });
    
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    row_index: 1,
                    row_span: 2,
                    col_index: 0,
                    col_span: 1,
                    layout_type: LayoutType::Grid,
                    grid_rows: vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)],
                    grid_cols: vec![Units::Stretch(1.0), Units::Stretch(1.0), Units::Stretch(1.0)],
                    ..Default::default()
                },
                material: materials.add(Color::PINK.into()),
                ..Default::default()
            }).with_node_children(|parent|{
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        row_index: 0,
                        row_span: 1,
                        col_index: 0,
                        col_span: 2,
                        ..Default::default()
                    },
                    material: materials.add(Color::OLIVE.into()),
                    ..Default::default()
                });
        
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        row_index: 0,
                        row_span: 1,
                        col_index: 2,
                        col_span: 1,
                        ..Default::default()
                    },
                    material: materials.add(Color::VIOLET.into()),
                    ..Default::default()
                });
        
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        row_index: 1,
                        row_span: 2,
                        col_index: 0,
                        col_span: 1,
                        ..Default::default()
                    },
                    material: materials.add(Color::TOMATO.into()),
                    ..Default::default()
                });
        
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        row_index: 1,
                        row_span: 1,
                        col_index: 1,
                        col_span: 2,
                        ..Default::default()
                    },
                    material: materials.add(Color::NAVY.into()),
                    ..Default::default()
                });
        
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        row_index: 3,
                        row_span: 1,
                        col_index: 0,
                        col_span: 1,
                        ..Default::default()
                    },
                    material: materials.add(Color::CRIMSON.into()),
                    ..Default::default()
                });
        
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        row_index: 2,
                        row_span: 2,
                        col_index: 1,
                        col_span: 1,
                        ..Default::default()
                    },
                    material: materials.add(Color::SEA_GREEN.into()),
                    ..Default::default()
                });
        
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        row_index: 2,
                        row_span: 2,
                        col_index: 2,
                        col_span: 1,
                        ..Default::default()
                    },
                    material: materials.add(Color::TEAL.into()),
                    ..Default::default()
                });
            });
    
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    row_index: 1,
                    row_span: 1,
                    col_index: 1,
                    col_span: 2,
                    ..Default::default()
                },
                material: materials.add(Color::SALMON.into()),
                ..Default::default()
            });
    
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    row_index: 3,
                    row_span: 1,
                    col_index: 0,
                    col_span: 1,
                    ..Default::default()
                },
                material: materials.add(Color::CYAN.into()),
                ..Default::default()
            });
    
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    row_index: 2,
                    row_span: 2,
                    col_index: 1,
                    col_span: 1,
                    ..Default::default()
                },
                material: materials.add(Color::YELLOW.into()),
                ..Default::default()
            });
    
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    row_index: 2,
                    row_span: 2,
                    col_index: 2,
                    col_span: 1,
                    ..Default::default()
                },
                material: materials.add(Color::DARK_GRAY.into()),
                ..Default::default()
            });
        });

        parent.spawn_bundle(NodeBundle {
            style: Style {
                row_index: 1,
                row_span: 1,
                col_index: 1,
                col_span: 2,
                ..Default::default()
            },
            material: materials.add(Color::NAVY.into()),
            ..Default::default()
        });

        parent.spawn_bundle(NodeBundle {
            style: Style {
                row_index: 3,
                row_span: 1,
                col_index: 0,
                col_span: 1,
                ..Default::default()
            },
            material: materials.add(Color::CRIMSON.into()),
            ..Default::default()
        });

        parent.spawn_bundle(NodeBundle {
            style: Style {
                row_index: 2,
                row_span: 2,
                col_index: 1,
                col_span: 1,
                ..Default::default()
            },
            material: materials.add(Color::SEA_GREEN.into()),
            ..Default::default()
        });

        parent.spawn_bundle(NodeBundle {
            style: Style {
                row_index: 2,
                row_span: 2,
                col_index: 2,
                col_span: 1,
                ..Default::default()
            },
            material: materials.add(Color::TEAL.into()),
            ..Default::default()
        });
    });

    /*
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
            // right vertical fill
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                ..Default::default()
            });
            // absolute positioning
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(210.0),
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        border: Rect::all(Val::Px(20.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.4, 0.4, 1.0).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        material: materials.add(Color::rgb(0.8, 0.8, 1.0).into()),
                        ..Default::default()
                    });
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                material: materials.add(Color::rgb(1.0, 0.3, 0.3).into()),
                                ..Default::default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(40.0),
                                        bottom: Val::Px(40.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
                                ..Default::default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(60.0),
                                        bottom: Val::Px(60.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                material: materials.add(Color::rgb(1.0, 0.7, 0.7).into()),
                                ..Default::default()
                            });
                            // alpha test
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(80.0),
                                        bottom: Val::Px(80.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                material: materials.add(Color::rgba(1.0, 0.9, 0.9, 0.4).into()),
                                ..Default::default()
                            });
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(500.0), Val::Auto),
                            ..Default::default()
                        },
                        material: materials
                            .add(asset_server.load("branding/bevy_logo_dark_big.png").into()),
                        ..Default::default()
                    });
                });
        });
    */
}
