use crate::rpc::handler::table_handler;
use atlas_core::atlas_rpc_dispatch;
use atlas_scheme::module_method::holdem_method;

atlas_rpc_dispatch! {
    module holdem_bind {
        holdem_method::GetTableRpc => table_handler::get_table,
        holdem_method::SitTableRpc => table_handler::sit_table,
        holdem_method::LeaveTableRpc => table_handler::leave_table,
        holdem_method::GameActRpc => table_handler::game_act,
    }
}