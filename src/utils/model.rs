use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Session {
    pub sid: Uuid,
    pub sess: String,
    pub expire: DateTime<Utc>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    pub id: Uuid,
    pub pwHash: String,
    pub pwSalt: String,
    pub email: String,
    pub firstName: String,
    pub lastName: String,
    pub primaryAddressId: Option<Uuid>,
    pub primaryPaymentId: Option<Uuid>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Address {
    pub id: Uuid,
    pub userId: Uuid,
    pub firstName: String,
    pub lastName: String,
    pub address1: String,
    pub address2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Card {
    pub id: Uuid,
    pub userId: Uuid,
    pub cardType: String,
    pub provider: String,
    pub cardNo: String,
    pub cvv: String,
    pub expMonth: i32,
    pub expYear: i32,
    pub billingAddressId: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Order {
    pub id: Uuid,
    pub userId: Uuid,
    pub status: String,
    pub shippingAddressId: Uuid,
    pub billingAddressId: Uuid,
    pub paymentId: Uuid,
    pub stripeChargeId: String,
    pub amountCharged: f64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OrderItem {
    pub orderId: Uuid,
    pub productId: Uuid,
    pub quantity: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Cart {
    pub id: Uuid,
    pub userId: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CartItem {
    pub cartId: Uuid,
    pub productId: Uuid,
    pub quantity: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub category: Option<String>,
    pub inStock: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}