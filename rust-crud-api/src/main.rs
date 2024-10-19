#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
struct Item {
    id: i32,
    name: String,
    quantity: i32,
}

#[derive(Deserialize)]
struct NewItem {
    name: String,
    quantity: i32,
}

#[post("/items", data = "<item>")]
async fn create_item(pool: &rocket::State<sqlx::PgPool>, item: Json<NewItem>) -> Json<Item> {
    let result = sqlx::query_as!(
        Item,
        "INSERT INTO items (name, quantity) VALUES ($1, $2) RETURNING id, name, quantity",
        item.name,
        item.quantity
    )
    .fetch_one(pool.inner())
    .await
    .unwrap();
    Json(result)
}

#[get("/items")]
async fn get_items(pool: &rocket::State<sqlx::PgPool>) -> Json<Vec<Item>> {
    let items = sqlx::query_as!(Item, "SELECT id, name, quantity FROM items")
        .fetch_all(pool.inner())
        .await
        .unwrap();
    Json(items)
}

#[put("/items/<id>", data = "<item>")]
async fn update_item(
    pool: &rocket::State<sqlx::PgPool>,
    id: i32,
    item: Json<NewItem>,
) -> Json<Item> {
    let result = sqlx::query_as!(
        Item,
        "UPDATE items SET name = $1, quantity = $2 WHERE id = $3 RETURNING id, name, quantity",
        item.name,
        item.quantity,
        id
    )
    .fetch_one(pool.inner())
    .await
    .unwrap();
    Json(result)
}

#[delete("/items/<id>")]
async fn delete_item(pool: &rocket::State<sqlx::PgPool>, id: i32) -> &'static str {
    sqlx::query!("DELETE FROM items WHERE id = $1", id)
        .execute(pool.inner())
        .await
        .unwrap();
    "Item deleted"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .manage(init_db())
        .mount("/", routes![create_item, get_items, update_item, delete_item])
}

async fn init_db() -> sqlx::PgPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}