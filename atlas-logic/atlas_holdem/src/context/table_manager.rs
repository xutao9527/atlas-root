use crate::model::table::Table;
use dashmap::DashMap;
use std::sync::Arc;

pub struct TableManager {
    tables: DashMap<String, Arc<Table>>,
}

impl TableManager {

    /// 创建一个空的 TableManager
    pub fn new() -> Self {
        Self {
            tables: DashMap::new(),
        }
    }

    /// 初始化 N 张桌子（服务启动时调用）
    pub fn init_tables(&self, count: usize) {
        for i in 0..count {
            let table_id = format!("table-{}", i + 1);
            let table = Arc::new(Table::new_six(10, 20));
            self.tables.insert(table_id, table);
        }
    }

    /// 获取指定 table
    pub fn get(&self, table_id: &str) -> Option<Arc<Table>> {
        self.tables.get(table_id).map(|e| e.value().clone())
    }

    /// 返回所有 table（只读场景）
    pub fn all(&self) -> Vec<Arc<Table>> {
        self.tables.iter().map(|e| e.value().clone()).collect()
    }

    /// 添加 table（动态扩容）
    pub fn insert(&self, table: Table) {
        self.tables
            .insert(table.id.clone(), Arc::new(table));
    }

    /// 移除 table
    pub fn remove(&self, table_id: &str) -> Option<Arc<Table>> {
        self.tables.remove(table_id).map(|(_, v)| v)
    }

    /// 当前 table 数量
    pub fn len(&self) -> usize {
        self.tables.len()
    }
}
