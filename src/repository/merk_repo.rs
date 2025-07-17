use deadpool_postgres::Pool;
use crate::service::Item;

pub async fn fetch_all_items(pool: &Pool) -> Result<Vec<Item>, tokio_postgres::Error> {
    let client = pool.get().await?;
    let stmt = client.prepare("SELECT c_nm, b_enabled FROM merk_master").await?;
    let rows = client.query(&stmt, &[]).await?;

    let items: Vec<Item> = rows.iter().map(|row| Item {
        c_nm: row.get(0),
        b_enabled: row.get(1),
    }).collect();

    Ok(items)
}