use crate::context::table_manager;
use atlas_core::net::rpc::packet_payload::AtlasRpcPayload;
use atlas_scheme::proto::holdem::holdem_model::{GetTableReq, GetTableResp, TableView};

pub async fn get_table(_req: GetTableReq) -> AtlasRpcPayload<GetTableResp> {
    let table = table_manager();
    let mut table_views = Vec::new();
    for table in table.all() {
        let players = table.seats.iter().map(|seat| seat.is_some()).collect();
        let table_view = TableView {
            id: table.id.clone(),
            seats: players,
            small_blind_amount: table.small_blind_amount,
            big_blind_amount: table.big_blind_amount,
        };
        table_views.push(table_view);
    }

    AtlasRpcPayload::Ok(GetTableResp {
        tables: table_views,
    })
}
