use bevy::asset::AssetServer;
use bevy::audio::AudioPlayer;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::enemy::components::Enemy;
use crate::game::enemy::*;
use crate::game::Player;
use crate::game::star::events::*;
use crate::game::star::resources::*;

