use std::ops::Range;

use bevy_ecs::{
    prelude::{Component, EventReader},
    system::Query,
};
use bevy_text::Text;
use bevy_window::ReceivedCharacter;

use crate::Node;

const BACKSPACE: char = '\u{8}';
const ESCAPE: char = '\u{1b}';
const DELETE: char = '\u{7f}';

pub(crate) fn update_text_input(
    mut text_query: Query<(&mut Text, &TextInput)>,
    mut char_evr: EventReader<ReceivedCharacter>,
) {
    let chars = char_evr.iter().collect::<Vec<_>>();
    for (mut text, text_input) in text_query
        .iter_mut()
        .filter(|(_, text_input)| text_input.active)
    {
        for char in &chars {
            if char.char == BACKSPACE {
                // TODO: delete full unicode segments, not just characters.
                // delete last character from the last non-empty section
                if let Some(mut section) = text
                    .sections
                    .iter_mut()
                    .rev()
                    .find(|section| !section.value.is_empty())
                {
                    section.value.pop();
                }
            } else if char.char == DELETE {
                // TODO: Add support
            } else if char.char == ESCAPE {
                // TODO: add support
            } else if let Some(mut section) = text.sections.last_mut() {
                // process regular character
                section.value.push(char.char.clone());
            }
        }
    }
}

/// Adds input support to an entity with the [`Text`] component.
#[derive(Component)]
pub struct TextInput {
    /// Marks that the text input is active and it should receive text input.
    ///
    /// TODO: replace this with a more generic focus system.
    pub active: bool,
}
