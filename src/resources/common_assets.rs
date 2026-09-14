use bevy::prelude::*;
use std::collections::HashMap;
use std::string::String;

#[derive(Clone)]
pub enum ResourceType {
    FontHandle(Handle<Font>),
    ImageHandle(Handle<Image>),
}

#[derive(Resource, Default, Clone)]
pub struct CommonAssets {
    pub resource_map: HashMap<String, ResourceType>,
    /// The `graph_attrs.font` URL the `"default_font"` handle currently reflects
    /// (`None` means it's still the hardcoded app-wide default) - lets
    /// `systems::resource_loader` tell "theme didn't set a font" apart from
    /// "theme set the same font as last time" and "theme changed font",
    /// without re-issuing an `asset_server.load` on every single graph reload.
    pub theme_font_url: Option<String>,
}

#[derive(Default, PartialEq, Clone)]
pub enum LoadingStateOpt {
    #[default]
    Loading,
    Ready,
}
#[derive(Resource, PartialEq, Default, Clone)]
pub struct LoadingState {
    pub state: LoadingStateOpt,
}
