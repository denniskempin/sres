#[derive(Clone, Copy, PartialEq, Debug, strum::Display, strum::EnumString)]
pub enum NativeVectorTable {
    #[strum(serialize = "cop")]
    Cop = 0xFFE4,
    #[strum(serialize = "break", serialize = "brk")]
    Break = 0xFFE6,
    #[strum(serialize = "nmi")]
    Nmi = 0xFFEA,
    #[strum(serialize = "irq")]
    Irq = 0xFFEE,
}
