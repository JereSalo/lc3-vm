#[derive(PartialEq, Clone, Copy)]
pub enum ConditionFlag {
    Pos = 1 << 0, // 001
    Zro = 1 << 1, // 010
    Neg = 1 << 2, // 100
}
