// TODO: figure out what to do when cfg(lighting) is not enabled
use UnimplementedType;
use List;
use devices;

macro_rules! default_settings {
    () => {
        Settings
        { layout: KeyBoardLayout::Qwerty(QwertyKind::USInternational),
          devices: None,
          #[cfg(lighting)] lighting: UnimplementedType!()
        }

    };
}

/// This struct contains all the settings needed during runtime
pub struct Settings {
    layout: KeyBoardLayout,
    pub devices: Option<devices::DeviceSettings>,
    #[cfg(lighting)] lighting: lighting::LightingSettings,
}

enum DvorakKind {
    // Programmer's Dvorak, my favourite
    Programmer,
    //    Regular,
    //    Left,
    //    Right
}

enum QwertyKind {
    USInternational,
    //    Dutch
    //    UK,
    //    German
}

enum KeyBoardLayout {
    Qwerty(QwertyKind),
    Dvorak(DvorakKind),
    // Azerty,
    // ?
}

pub fn load_settings() -> Settings {
    let defaults = default_settings!();
    defaults
}
