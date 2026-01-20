use crate::model::card::AtlasCard;
use atlas_scheme::proto::holdem::types::PlayerView;
use rs_poker::core::Rank;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,       // 玩家全局唯一 ID
    pub nickname: String, // 玩家昵称
    pub balance: u64,     // 玩家当前可用筹码

    pub hand_cards: [Option<AtlasCard>; 2], // 玩家手牌（底牌），每人 2 张
    pub cards_str: String,                  // 玩家手牌 (调试字符串)
    pub cards_rank: Option<Rank>,           // 牌力值

    pub sit_out: bool, // 是否下局不玩（离桌）
    pub win: bool,     // 是不是赢家

    pub is_active: bool, // 是否仍在牌局中（Fold 后为 false）
    pub has_acted: bool, // 本下注轮是否已经行动过
    pub acted_str: String, // 行动结果展示

    pub is_all_in: bool, // 是否已all_in
    pub street_bet: u64, // 本下注轮（当前 street）中已投入的筹码
    pub total_bet: u64,  // 总投入筹码
}



impl From<&Player> for PlayerView {
    fn from(player:  &Player) -> Self {
        PlayerView {
            id: player.id.clone(),
            nickname: player.nickname.to_string(),
            balance: player.balance,
            sit_out: player.sit_out,
            win: player.win,
            is_active: player.is_active,
            has_acted: player.has_acted,
            is_all_in: player.is_all_in,
            street_bet: player.street_bet,
            total_bet: player.total_bet,
        }
    }
}

impl From<Player> for PlayerView {
    fn from(player: Player) -> Self {
        (&player).into()
    }
}