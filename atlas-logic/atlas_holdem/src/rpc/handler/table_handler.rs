use crate::context::{get_db, table_manager};
use crate::model::player::Player;
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_core::net::rpc::packet_payload::{AtlasRpcPayload, AtlasWireError};
use atlas_scheme::model::atlas_user;
use atlas_scheme::proto::holdem::holdem_model::*;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use ulid::Ulid;

pub async fn get_table_list(_req: AtlasWireMessage<GetTableListReq>) -> AtlasRpcPayload<GetTableListResp> {
    let mut table_views = Vec::new();
    for table in table_manager().all() {
        let table = table.read().await;
        let seats = table.seats.iter()
            .map(|seat| seat.as_ref().map(|p| p.nickname.clone()))
            .collect();
        let table_view = TableListView {
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

pub async fn get_table_info(req: AtlasWireMessage<GetTableInfoReq>) -> AtlasRpcPayload<GetTableInfoResp> {
    let resp = match table_manager().get(&req.payload.table_id) {
        Some(table) => {
            let table_data = table.read().await;
            AtlasRpcPayload::Ok(GetTableInfoResp {
                table_id: table_data.id.to_string(),
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

pub async fn sit_table(req: AtlasWireMessage<SitTableReq>) -> AtlasRpcPayload<SitTableResp> {
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
        id: user.id,                // 之后用 uid
        nickname: user.name,          // 之后从用户表来
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
                table_id: table.id.clone(),
                message: Some("sit table success".into()),
            })
        }
        Err(e) => {
            AtlasRpcPayload::Err(e.into())
        }
    }
}

pub async fn leave_table(req: AtlasWireMessage<LeaveTableReq>) -> AtlasRpcPayload<LeaveTableResp> {
    //  ===== 0. 获得自己ID =====
    let user_id = Ulid::from_bytes(req.header.uid).to_string();

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

pub async fn game_act(_req: AtlasWireMessage<GameActReq>) -> AtlasRpcPayload<GameActResp> {
    AtlasRpcPayload::Ok(GameActResp {
        ok: false,
        message: None,
    })
}