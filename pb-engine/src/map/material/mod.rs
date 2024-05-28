use bevy_asset::AssetEvent;
use bevy_ecs::event::EventReader;

use self::def::MaterialDefinitions;

mod def;

pub fn load_materials(mut asset_events: EventReader<AssetEvent<MaterialDefinitions>>) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            
        }
    }
}
