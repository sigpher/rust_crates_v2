// use json::News;
// use json::Photo;
use sqlx::{Row, SqlitePool};
// use std::{collections::HashMap, error::Error, fs, io::Write};
use std::{error::Error, fs, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let client = reqwest::Client::new();
    // let url = "https://jsonplaceholder.typicode.com/albums/1/photos";
    // let resp = client.get(url).send().await?;
    // let body = resp.text().await?;
    // let photos: Vec<Photo> = serde_json::from_str(&body).unwrap();
    let pool = SqlitePool::connect("test.db").await?;
    // for photo in photos {
    //     println!("albumId: {}", photo.album_id);
    //     println!("id: {}", photo.id);
    //     println!("title: {}", photo.title);
    //     println!("url: {}", photo.url);
    //     println!("thumbnailUrl: {}", photo.thumbnail_url);
    //     insert_data(&pool, photo).await?;
    // }
    list_data(&pool).await?;
    // get_news_from_netease().await?;
    Ok(())
}

// async fn insert_data<'a>(pool: &SqlitePool, data: Photo<'a>) -> Result<(), Box<dyn Error>> {
//     let mut conn = pool.acquire().await?;
//     let sql = r#"
//     INSERT INTO photos (album_id, id, title, url, thumbnail_url) VALUES (?,?,?,?,?)
//     "#;
//     let ret = sqlx::query(sql)
//         .bind(data.album_id)
//         .bind(data.id)
//         .bind(data.title)
//         .bind(data.url)
//         .bind(data.thumbnail_url)
//         .execute(&mut conn)
//         .await?
//         .last_insert_rowid();
//     print!("{ret}");
//     Ok(())
// }

async fn list_data<'a>(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    // let mut conn = pool.acquire().await?;
    let sql = r#"SELECT id, title, url FROM photos"#;
    let ret = sqlx::query(sql).fetch_all(pool).await?;
    // for (_, row) in ret.iter().enumerate() {
    //     println!(
    //         "{},{},{}",
    //         row.get::<u8, &str>("id"),
    //         row.get::<&str, &str>("url"),
    //         row.get::<&str, &str>("title"),
    //     );
    // }

    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("ret.csv")
        .unwrap();
    for (_, row) in ret.iter().enumerate() {
        f.write(
            format!(
                "{},{},{}\n",
                row.get::<u8, &str>("id"),
                row.get::<&str, &str>("url"),
                row.get::<&str, &str>("title"),
            )
            .as_bytes(),
        )
        .unwrap();
    }
    Ok(())
}

// async fn get_news_from_netease() -> Result<(), Box<dyn Error>> {
//     let client = reqwest::Client::new();
//     let url = "http://c.m.163.com/nc/article/headline/T1348647853363/0-40.html";
//     let resp = client.get(url).send().await?;
//     let body = resp.text().await?;
//     let news: HashMap<&str,Vec<News>>= serde_json::from_str(&body).unwrap();

//     // println!("{:?}",news.values());

//     Ok(())
// }
