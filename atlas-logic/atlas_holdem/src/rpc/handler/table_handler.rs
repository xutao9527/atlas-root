use ulid::Ulid;
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use crate::context::table_manager;
use crate::model::player::Player;
use atlas_core::net::rpc::packet_payload::{AtlasRpcPayload, AtlasWireError};
use atlas_scheme::proto::holdem::holdem_model::*;

pub async fn get_table(_req: AtlasWireMessage<GetTableReq>) -> AtlasRpcPayload<GetTableResp> {
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
    AtlasRpcPayload::Ok(GetTableResp {
        tables: table_views,
    })
}

pub async fn sit_table(req: AtlasWireMessage<SitTableReq>) -> AtlasRpcPayload<SitTableResp> {
    // ===== 1. 获取桌子 =====
    let table = match table_manager().get(&req.payload.table_id) {
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
    let seat_index = req.payload.seat_index as usize;
    // ===== 3. 写锁：真正修改桌子 =====
    let mut table = table.write().await;
    let player = Player {
        id: Ulid::new().to_string(),                // 之后用 uid
        nickname: Ulid::new().to_string(),          // 之后从用户表来
        balance: req.payload.buy_in,
        hand_cards: [None, None],
        cards_str: String::new(),
        sit_out: false,
        win: false,
        cards_rank: None,
        is_active: false,
        has_acted: false,
        is_all_in: false,
        street_bet: 0,
        total_bet: 0,
    };
    // ===== 7. 放入桌子 =====
    match table.sit(seat_index, player) {
        Ok(_) => {
            AtlasRpcPayload::Ok(SitTableResp {
                ok: true,
                message: Some("sit table success".into()),
            })
        }
        Err(e) => {
            AtlasRpcPayload::Err(e.into())
        }
    }
}

pub async fn leave_table(req: AtlasWireMessage<LeaveTableReq>) -> AtlasRpcPayload<LeaveTableResp> {
    // ===== 1. 获取桌子 =====
    let table = match table_manager().get(&req.payload.table_id) {
        Some(t) => t,
        None => {
            return AtlasRpcPayload::Err(AtlasWireError {
                code: 404,
                message: "table not found".into(),
                data: None,
            });
        }
    };
    let seat_index = req.payload.seat_index as usize;

    // ===== 2. 写锁：执行离桌逻辑 =====
    let mut table = table.write().await;

    match table.leave(seat_index) {
        Ok(_) => AtlasRpcPayload::Ok(LeaveTableResp {
            ok: true,
            message: Some("leave table success".into()),
        }),
        Err(e) => AtlasRpcPayload::Err(e.into()),
    }
}

pub async fn game_act(_req: AtlasWireMessage<GameActReq>) -> AtlasRpcPayload<GameActResp> {
    AtlasRpcPayload::Ok(GameActResp {
        ok: false,
        message: None,
    })
}