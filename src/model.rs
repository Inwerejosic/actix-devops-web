use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::ip_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IpLog {
    pub id: i32,
    pub original_ip: String,
    pub reversed_ip: String,
    pub created_at: chrono::DateTime<chrono::Utc>, // ✅ TIMESTAMPTZ maps to DateTime<Utc>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ip_logs)]
pub struct NewIpLog<'a> {
    pub original_ip: &'a str,
    pub reversed_ip: &'a str,
}
