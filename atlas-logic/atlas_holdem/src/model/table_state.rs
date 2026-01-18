#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableState {
    Waiting,    // 空闲状态：
    Preparing,  // 准备阶段：
    Battling,   // 对战阶段：
    Concluding, // 结算阶段：
}
