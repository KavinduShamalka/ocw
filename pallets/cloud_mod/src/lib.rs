use reqwest::{Client, Error};
use serde_json::{Value, json};

pub async fn create_bucket() -> Result<(), Error> {

    let new_bucket : Value =  Client::new()
        .post("https://storage.googleapis.com/storage/v1/b?project=intern-storage-apis")
        .header("Authorization", "Bearer ya29.c.c0AY_VpZidKZon8utGWNP1syvJ9rk1vRIm6HWXfIYvI_b9YeQ96-uepIX_w9dlLU0MnnQHhMKkZ5KdBuhiQC3zYhdU8rQY4uYTAzrYugqHx7FD-bC2FAUljSnQTU2Vs9qA70qY_9KlNKnnnZyq2Yp-kw6RT-xouCR7JyBFMBvG_CPiSrhJAmdQf7mVQkPSGFx4mxzyBxwjrKDr5dFigYWNH-m6-8tXnlPrdyQ57zZ6dIrxc0nYwtghV02WqmoWmqGwawXXlwu34oqLU7t5urIOrSY1rqpqnPl8AURTO3EfUrnA5wvUh4p_UBsQ-zpbkBiCyeGAW4WKMV2VlSoVij9zSpAHygn8BUcbwrUyJz4UEVuA0BRQl7LlmTcplFZDfPKYj96B1QE399Dkk3nFsmkhn4BXmlxcfUztmi07h73q0nxnrkrSv9MjaIj95ni8RUjv1J61Btssik64t5OtFRridxMI1ak-cb3oFOF0pYp9v6O_0lIb8MRUh5MJ7tpZ5M5zSZvMluYt7c-1rrIsi-ntfBzRbVkI0-c8frF7Rs4r2Fus0ouWoV2swkk2ZgtIcpjnte6cWg6_MIfjlpgj54V4_qjZVBbii0e6kXOYfWhj-1Qkm33apcORR4V84cISIRiyijXkdhiJt9jrz832Xn344Ui6uWzj-5iOolXjZh6h94Q_h9V1u7ZFiImQyu08QWbMgYRXxvx0y8sZJzwaaiZsIad1o3dzsBp11fJVqrrfusQ0fOtZF7ZeweRRggrtQpV8ma6M1qbZInS6oZss-juZtVpr-9d4MYwoam1bc9lOmgUxiUhr_4Ypyhhlm1_Rpbi_ZMhu0eBUfpZMmw88I1diFjnfee-V6Vi4Jb4WO2-9wBljlirQOVUW8tr8zXsMp3t9hOlueVskjYVvQiZ_hkV6ykIzWupcSwhiI1yY-utVsuqBBdRB2aqqiWBYb5QncxxrU6gZbsyJVdilfm0sU6vhXdpWWy7gW318Iqq9tfzSZ86ogS8tqj5xIZr")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&json!({
            "name": "intern-fcx",
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", new_bucket);
    Ok(())
}
