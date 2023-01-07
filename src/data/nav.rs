use druid::Data;

#[derive(Clone, Copy, Debug, Data, PartialEq, Eq)]
pub enum Nav {
    Settings,
    CustomCarLoader,
    SkinManager,
    CargoSwap,
    ZSounds,
}

impl Default for Nav {
    fn default() -> Self {
        Nav::CustomCarLoader
    }
}
