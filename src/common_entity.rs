
use bevy::{prelude::{Bundle, AssetServer, Res, default, Color, ChildBuilder, ButtonBundle, BuildChildren, ImageBundle, TextBundle}, ui::{Style, Val, UiRect, JustifyContent, AlignItems, PositionType, UiImage}, text::TextStyle};


const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);


pub trait EntitySpawner {
    fn spawn_button(&mut self, bundle: impl Bundle, icon_image_path: &'static str, title: &str, asset_server: &Res<AssetServer>);
}

impl EntitySpawner for ChildBuilder<'_, '_, '_> {
    fn spawn_button(&mut self, bundle: impl Bundle, icon_image_path: &'static str, title: &str, asset_server: &Res<AssetServer>) {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_style = Style {
            width: Val::Px(30.0),
            height: Val::Auto,
            position_type: PositionType::Relative,
            ..default()
        };
        let button_text_style = TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: TEXT_COLOR,
        };
        
        self.spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            bundle,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load(icon_image_path);
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(title, button_text_style));
                        });
    }
}