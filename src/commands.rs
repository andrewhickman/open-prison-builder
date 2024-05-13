use crate::ui::ButtonCommand;

pub const PLAY: ButtonCommand = ButtonCommand::new("prison_builder::main_menu::play");
pub const OPEN_BEVY: ButtonCommand = ButtonCommand::new("prison_builder::main_menu::open_bevy");

pub const BUILD_MENU: ButtonCommand =
    ButtonCommand::new("prison_builder::menu_bar::open_build_menu");
pub const VIEW_MENU: ButtonCommand = ButtonCommand::new("prison_builder::menu_bar::open_view_menu");
pub const MANAGE_MENU: ButtonCommand =
    ButtonCommand::new("prison_builder::menu_bar::open_manage_menu");

pub const SELECT_MATERIAL: ButtonCommand =
    ButtonCommand::new("prison_builder::menu_bar::build::select_material");
