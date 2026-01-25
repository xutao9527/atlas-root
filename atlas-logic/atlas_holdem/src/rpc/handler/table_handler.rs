use crate::context::{get_db, table_manager};
use crate::model::player::Player;
use atlas_scheme::model::atlas_user;
use atlas_scheme::proto::holdem::rpc::*;
use atlas_scheme::proto::holdem::types::TableView;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use ulid::Ulid;
use atlas_core::net::protocol::frame::AtlasFrame;
use atlas_core::net::protocol::frame_body_rpc::{AtlasRpcPayload, AtlasWireError};

pub async fn get_table_list(_req: AtlasFrame<GetTableListReq>) -> AtlasRpcPayload<GetTableListResp> {
    let mut table_views = Vec::new();
    for table in table_manager().all() {
        let table = table.read().await;
        let seats = table.seats.iter()
            .map(|seat| seat.as_ref().map(|p| p.nickname.clone()))
            .collect();
        let table_view = TableView {
            id: table.id.clone(),
            seats,
            small_blind_amount: table.small_blind_amount,
            big_blind_amount: table.big_blind_amount,
        };
        table_views.push(table_view);
    }
    AtlasRpcPayload::Ok(GetTableListResp {
        tables: table_views,
    })
}

pub async fn get_table_info(req: AtlasFrame<GetTableInfoReq>) -> AtlasRpcPayload<GetTableInfoResp> {
    let uid = Ulid::from_bytes(req.header.uid).to_string();

    let resp = match table_manager().get(&req.body.table_id) {
        Some(table) => {
            let table = table.read().await;
            // 1️⃣ 找到自己的 seat
            let seat_index = match table.find_seat_index_by_player_id(&uid) {
                Some(i) => i,
                None => {
                    return AtlasRpcPayload::Err(AtlasWireError {
                        code: 403,
                        message: "player not seated at this table".into(),
                        data: None,
                    });
                }
            };
            // 2️⃣ 拿到 Player（引用）
            let player = match table.seats.get(seat_index).and_then(|s| s.as_ref()) {
                Some(player) => player,
                None => {
                    return AtlasRpcPayload::Err(AtlasWireError {
                        code: 500,
                        message: "seat exists but player missing".into(),
                        data: None,
                    });
                }
            };
            // 3️⃣ 构造响应
            AtlasRpcPayload::Ok(GetTableInfoResp {
                id: table.id.clone(),
                seats: table.seats.iter().map(|p|p.as_ref().map(Into::into)).collect(),
                state: table.state.into(),
                hand_id: table.hand_id.clone(),
                street: table.street.into(),
                small_blind_amount: table.small_blind_amount,
                big_blind_amount: table.big_blind_amount,
                pot: table.pot,
                current_bet: table.current_bet,
                dealer_pos: table.dealer_pos,
                small_blind_pos: table.small_blind_pos,
                big_blind_pos: table.big_blind_pos,
                current_turn: table.current_turn,
                current_turn_act: table.current_turn_act.clone().into(),
                last_raiser_pos: table.last_raiser_pos,
                // 公共牌
                community_cards: table.community_cards.map(|opt| {
                    opt.as_ref().map(Into::into)
                }),
                // ✅ 手牌（关键点）
                hand_cards: player.hand_cards.map(|opt| {
                    opt.as_ref().map(Into::into)
                }),
                seat_index
            })
        }
        None => {
            return AtlasRpcPayload::Err(AtlasWireError {
                code: 404,
                message: "table not found".into(),
                data: None,
            });
        }
    };
    resp
}

pub async fn sit_table(req: AtlasFrame<SitTableReq>) -> AtlasRpcPayload<SitTableResp> {
    //  ===== 0. 获得自己 =====
    let user = match atlas_user::Entity::find()
        .filter(atlas_user::Column::Id.eq(Ulid::from_bytes(req.header.uid).to_string()))
        .one(get_db())
        .await {
        Ok(Some(u)) => { u }
        _ => {
            return AtlasRpcPayload::Err(AtlasWireError {
                code: 404,
                message: "user not found".into(),
                data: None,
            });
        }
    };
    // ===== 1. 获取桌子 =====
    let table = match table_manager().get(&req.body.table_id) {
        Some(table) => {
            table
        }
        None => {
            return AtlasRpcPayload::Err(AtlasWireError {
                code: 404,
                message: "table not found".into(),
                data: None,
            });
        }
    };
    let seat_index = req.body.seat_index as usize;
    // ===== 3. 写锁：真正修改桌子 =====
    let mut table = table.write().await;
    let player = Player {
        id: user.id,                // 之后用 uid
        nickname: user.name,          // 之后从用户表来
        balance: req.body.buy_in,
        hand_cards: [None, None],
        cards_str: String::new(),
        sit_out: false,
        win: false,
        cards_rank: None,
        is_active: false,
        has_acted: false,
        acted_view: "".to_string(),
        is_all_in: false,
        street_bet: 0,
        total_bet: 0,
    };
    // ===== 4. 放入桌子 =====
    match table.sit(seat_index, player) {
        Ok(_) => {
            AtlasRpcPayload::Ok(SitTableResp {
                ok: true,
                table_id: table.id.clone(),
                message: Some("sit table success".into()),
            })
        }
        Err(e) => {
            AtlasRpcPayload::Err(e.into())
        }
    }
}

pub async fn leave_table(req: AtlasFrame<LeaveTableReq>) -> AtlasRpcPayload<LeaveTableResp> {
    //  ===== 0. 获得自己ID =====
    let user_id = Ulid::from_bytes(req.header.uid).to_string();
    // ===== 1. 获取桌子 =====
    let table = match table_manager().get(&req.body.table_id) {
        Some(t) => t,
        None => {
            return AtlasRpcPayload::Err(AtlasWireError {
                code: 404,
                message: "table not found".into(),
                data: None,
            });
        }
    };
    // ===== 2. 写锁：执行离桌逻辑 =====
    let mut table = table.write().await;

    match table.leave(&user_id) {
        Ok(_) => AtlasRpcPayload::Ok(LeaveTableResp {
            ok: true,
            message: Some("leave table success".into()),
        }),
        Err(e) => AtlasRpcPayload::Err(e.into()),
    }
}

pub async fn game_act(req: AtlasFrame<GameActReq>) -> AtlasRpcPayload<GameActResp> {
    //  ===== 0. 获得自己ID =====
    let uid = Ulid::from_bytes(req.header.uid).to_string();
    // ===== 1. 获取桌子 =====
    let table = match table_manager().get(&req.body.table_id) {
        Some(t) => t,
        None => {
            return AtlasRpcPayload::Err(AtlasWireError {
                code: 404,
                message: "table not found".into(),
                data: None,
            });
        }
    };
    // ===== 2. 写锁：执行离桌逻辑 =====
    let mut table = table.write().await;
    // ===== 3. 调用 =====
    match table.act(uid, req.body.act.into()) {
        Ok(_) => AtlasRpcPayload::Ok(GameActResp {
            ok: true,
            message: Some("player act success".into()),
        }),
        Err(e) => AtlasRpcPayload::Err(e.into()),
    }

}

pub async fn game_start(req: AtlasFrame<GameStartReq>) -> AtlasRpcPayload<GameStartResp> {
    // ===== 1. 获取桌子 =====
    let table = match table_manager().get(&req.body.table_id) {
        Some(t) => t,
        None => {
            return AtlasRpcPayload::Err(AtlasWireError {
                code: 404,
                message: "table not found".into(),
                data: None,
            });
        }
    };
    // ===== 2. 写锁：执行离桌逻辑 =====
    let mut table = table.write().await;

    // ===== 3. 调用 =====
    match table.start() {
        Ok(_) => AtlasRpcPayload::Ok(GameStartResp {
            ok: true,
            message: Some("start game success".into()),
        }),
        Err(e) => AtlasRpcPayload::Err(e.into()),
    }

}