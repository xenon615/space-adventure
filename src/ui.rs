use bevy::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_event::<RegisterWidgets>();
        app.add_event::<UpdateWidgets>();
        app.add_systems(Update, register_widgets.run_if(on_event::<RegisterWidgets>()));
        app.add_systems(Update, update_widget.run_if(on_event::<UpdateWidgets>()));
    }
}

// ---

#[derive(Debug)]
pub struct WidgetRegData{
    pub parent: ULayout,
    pub wtype: WType,
    pub start: i16,
    pub span: u16,
    pub key: &'static str,
    pub label: &'static str,
    pub image: Option<Handle<Image>>
}

#[derive(Event, Debug)]
pub struct RegisterWidgets(pub Vec<WidgetRegData>); 

#[derive(Debug)]
pub struct WidgetUpdateData {
    key: &'static str,
    color: Option<Color>,
    value: f32
}

impl WidgetUpdateData {
    pub fn from_key_value(key: &'static str, value: f32) -> Self {
        Self {key, value, color: None}
    }

    pub fn from_key_value_color(key: &'static str, value: f32, color: Color) -> Self {
        Self {key, value, color: Some(color)}
    }
}

#[derive(Event, Debug)]
pub struct UpdateWidgets(pub Vec<WidgetUpdateData>);

#[derive(Component, PartialEq)]
pub struct WKey(pub &'static str);

// + LAYOUT====================

#[derive(Component, PartialEq, Debug)]
pub enum ULayout {
    Wrapper,
    Header,
    Footer,
    Body,
    SidebarLeft,
    Content,
    SidebarRight
}

#[derive(Component, PartialEq, Debug)]
pub enum WType {
    Text,
    Image
}

// - LAYOUT====================

fn spawn(
    mut commands: Commands,
) {

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Vw(100.),
                height:Val::Vh(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Stretch,
                ..default()
            },
            ..default() 
        },
        ULayout::Wrapper
    ))
    .with_children(|p| {
        p.spawn(( get_bar(), ULayout::Header));

        p.spawn((
            get_body(),
            ULayout::Body
        ))
        .with_children(|b| {
            b.spawn((get_sidebar(), ULayout::SidebarLeft));
            b.spawn((get_content(), ULayout::Content));
            b.spawn((get_sidebar(), ULayout::SidebarRight));
        });

        p.spawn((get_bar(), ULayout::Footer));
    });

} 

// ==================================================================

fn get_bar() -> NodeBundle {
    NodeBundle {
        style: Style {
            height:Val::Px(50.),
            width: Val::Percent(100.),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::percent(10, 10.),
            grid_template_rows: GridTrack::percent(100.),
            ..default()
        },
        background_color: Color::rgba(0., 1., 0., 0.005).into(),
        ..default()
    }
}

// ---

fn get_body () -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_grow: 2.0,
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        ..default()
    }
}

// ---

fn get_sidebar () -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::rgba(0., 0., 1., 0.005).into(),
        ..default()
    }
}

// ---

fn get_content () -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_grow: 2.0,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }
}

// ---

fn register_widgets(
    mut commands: Commands,
    mut evs: EventReader<RegisterWidgets>,
    nodes_q: Query<(Entity, &ULayout)>,
) {
    for ev in evs.read() {
        for w in ev.0.iter() { 
            for (node_ent, l)  in nodes_q.iter() {
                if *l == w.parent {

                    let widget_ent = commands.spawn(
                        NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(5.)),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                grid_column: GridPlacement::start_span(w.start, w.span),
                                justify_content: JustifyContent::Center,
                                align_items:AlignItems::Center, 
                                ..default()
                            },
                            ..default()
                        }
                    )
                    .with_children(| p | {
                        if w.wtype == WType::Text {
                            p.spawn(
                                TextBundle::from_section(
                                    format!("{}: ", w.label), 
                                    TextStyle {
                                        color: Color::GOLD.into(),
                                        font_size: 25.,
                                        ..default()
                                    } 
                                ).with_style(Style {
                                    flex_basis: Val::Percent(50.),
                                    ..default()
                                })
                            );
                            p.spawn((
                                TextBundle::from_section(
                                    "", 
                                    TextStyle {
                                        color: Color::GOLD.into(),
                                        font_size: 25.,
                                        ..default()
                                    } 
                                ).with_style(Style {
                                    flex_basis: Val::Percent(50.),
                                    ..default()
                                }),
                                WKey(w.key),
                                WType::Text
                            ));
    
                        } else {
                            p.spawn((
                                ImageBundle {
                                    style: Style {
                                        width: Val::Px(50.),
                                        height: Val::Px(50.),
                                        ..default()
                                    },
                                    image: UiImage::new(w.image.clone().unwrap()),
                                    ..default()
                                },
                                WKey(w.key),
                                WType::Image
                            ));
                        }
                    })
                    .id();
                    commands.entity(node_ent).add_child(widget_ent);
                } 
            }
        }
    }
    
}

// ---

fn update_widget(
    mut reader : EventReader<UpdateWidgets>,
    mut widget_q: Query<(Option< &mut Text>, &mut Transform, &WKey)> 
) {
    for ev in reader.read() {
        for w in ev.0.iter() {
            for (text_o, mut trans ,key) in widget_q.iter_mut() {
                if w.key == key.0 {
                    if let Some(mut text) = text_o {
                        text.sections[0].value = format!("{:.2}", w.value);
                        if let Some(c) = w.color {
                            text.sections[0].style.color = c;                        
                        }
                    } else {
                        trans.rotation = Quat::from_rotation_z(w.value) ;    
                    }
                }
            }
        }
    }
}

