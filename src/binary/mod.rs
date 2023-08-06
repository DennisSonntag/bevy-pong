use std::borrow::Cow;

use bevy::{asset::load_internal_binary_asset, prelude::*, reflect::TypeUuid};

pub const FONT_HANDLE: HandleUntyped =
	HandleUntyped::weak_from_u64(Font::TYPE_UUID, 436509473926038);

pub struct BinaryPlugin;

fn font_loader(bytes: &[u8], _: Cow<str>) -> Font {
	Font::try_from_bytes(bytes.to_vec()).expect("could not load font")
}

impl Plugin for BinaryPlugin {
	fn build(&self, app: &mut App) {
		load_internal_binary_asset!(app, FONT_HANDLE, "font/Roboto-Bold.ttf", font_loader);
	}
}
