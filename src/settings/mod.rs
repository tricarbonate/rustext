/*
 * Editor settings, modifiable by user
 */


pub struct Settings {
    number: LineNumbersSetting
}

impl Settings {
    pub fn default() -> Self {
        Self {
            number: LineNumbersSetting::None
        }
    }
}

enum LineNumbersSetting {
    None,
    Absolute,
    Relative
}
