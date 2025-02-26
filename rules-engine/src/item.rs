use core::fmt;
use std::sync::Arc;

use rusqlite::{Connection, Row};
use sea_query::{Expr, Iden, Query, SelectStatement, SimpleExpr, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::error::{VitruvianRulesEngineError, VitruvianRulesEngineResult};

//==============================================================================================
//        Rules Item Itentifier
//==============================================================================================

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct RulesItemIdentifier(Vec<String>);

impl RulesItemIdentifier {    
    pub fn next(&self, id: String) -> RulesItemIdentifier {
        let mut new = self.clone();
        new.0.push(id);
        new
    }
    
    pub fn peel(&self) -> RulesItemIdentifier {
        let mut new = self.clone();
        new.0.pop();
        new
    }
    
    pub fn push(&mut self, id: String) {
        self.0.push(id);
    }
    
    pub fn id(&self) -> String {
        self.0.join("::")
    }
}

impl Iden for RulesItemIdentifier {
    fn unquoted(&self,s: &mut dyn fmt::Write) {
        s.write_str("id");
    }
}

//==============================================================================================
//        Rules ITem
//==============================================================================================

pub trait RulesItem : Sized {
    
    type Table: ItemTable + 'static;
    
    fn from_row(row: &Row) -> VitruvianRulesEngineResult<Self>;
    
    fn map_value(&self, col : Self::Table) -> SimpleExpr;
    
    // fn values(&self) -> Vec<SimpleExpr> {
    //     Self::Table:: into_iter().map(|col| self.map_value(col)).collect()
    // }
    
    // fn map_values(&self) -> Vec<(Self::Table, SimpleExpr)> {
    //     Self::Table::list_columns().into_iter().map(|col| (col, self.map_value(col))).collect()
    // }
    
    // fn get<'s>(id : impl Into<&'s RulesItemIdentifier>, connection : &Connection) -> VitruvianRulesEngineResult<Self> {
    //     let id  : &'s RulesItemIdentifier = id.into();
    //     let id = id.id();
    //     Self::Table::create_table(connection)?;
        
    //     let (sql, values) = Query::select()
    //         .columns(Self::Table::list_columns())
    //         .from(Self::Table::table_variant())
    //         .and_where(Expr::col(Self::Table::id_iden()).eq(&id))
    //         .limit(1)
    //         .build_rusqlite(SqliteQueryBuilder)
    //     ;
        
    //     let mut stmt = connection.prepare(&sql)?;
    //     let mut rows = stmt.query(&*values.as_params())?;
    //     Self::from_row(rows.next()?.unwrap())
    // }
    
    // fn select() -> SelectStatement {
    //     Query::select()
    //         .columns(Self::Table::list_columns())
    //         .from(Self::Table::table_variant())
    //         .take()
    // }
    
    // fn select_col(columns : impl IntoIterator<Item = Self::Table>) -> SelectStatement {
    //     Query::select()
    //         .columns(columns)
    //         .from(Self::Table::table_variant())
    //         .take()
    // }
    
    // fn insert<'s>(&self, id : impl Into<&'s RulesItemIdentifier>, connection : &Connection) -> VitruvianRulesEngineResult<()> {
    //     let id : &'s RulesItemIdentifier = id.into();
    //     let mut columns : Vec<Self::Table> = vec![Self::Table::];
        
    //     let (sql, values) = Query::insert()
    //         .into_table(Self::Table::table_variant())
    //         .columns(Self::Table::list_columns())
    //         .values(self.values())?
    //         .build_rusqlite(SqliteQueryBuilder)
    //     ;
        
    //     let mut stmt = connection.prepare(&sql)?;
    //     stmt.execute(&*values.as_params())?;
    //     Ok(())
    // }
    
    // fn update(&self, connection : &Connection) -> VitruvianRulesEngineResult<()> {
    //     if self.id().is_none() { return Err(VitruvianRulesEngineError::UpdateRuleItemError("There is no id associated with item.".to_string()))}
    //     let id = self.id().unwrap();
        
    //     let (sql, values) = Query::update()
    //         .table(Self::Table::table_iden())
    //         .values(self.map_values())
    //         .and_where(Expr::col(Self::Table::id_iden()).eq(id.id()))
    //         .build_rusqlite(SqliteQueryBuilder)
    //     ;
        
    //     let mut stmt = connection.prepare(&sql)?;
    //     stmt.execute(&*values.as_params())?;
    //     Ok(())
    // }
}

//==============================================================================================
//        Rules Item Table
//==============================================================================================

pub trait ItemTable: IntoEnumIterator + Iden + Sized + Copy {
    fn create_table(db : &Connection) -> VitruvianRulesEngineResult<()>;
    
    fn table_iden() -> CustomIden;
    
    fn id_iden() -> CustomIden {
        CustomIden("feature".to_string())
    }
}

pub struct CustomIden(pub String);

impl Iden for CustomIden {
    fn unquoted(&self,s: &mut dyn fmt::Write) {
        write!(s, "{}", self.0);
    }
}