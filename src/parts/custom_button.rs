use super::super::{
    despawn_screen, DisplayQuality, GameState, Volume, HOVERED_BUTTON_COLOR,
    HOVERED_PRESSED_BUTTON_COLOR, MENU_FONT, MENU_TEXT_COLOR, NORMAL_BUTTON_COLOR,
    PRESSED_BUTTON_COLOR,
};

use bevy::prelude::*;

pub struct Button {
    style: Style,
    text_style: TextStyle,
}

impl Button {
    fn create(asset_server: Res<AssetServer>, text: String) -> Button {
        let button_style = Style {
            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        };

        let button_text_style = TextStyle {
            font: asset_server.load(MENU_FONT),
            font_size: 35.0,
            color: MENU_TEXT_COLOR,
        };

        Button {
            style: button_style,
            text_style: button_text_style,
        }
    }
}
