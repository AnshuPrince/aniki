use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditLedgerEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub delta: f64,
    pub reason: String,
    pub session_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsResponse {
    pub balance: f64,
    pub entries: Vec<CreditLedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInvoice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub external_id: Option<String>,
    pub amount_cents: i32,
    pub currency: String,
    pub pdf_url: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
