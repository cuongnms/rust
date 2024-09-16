use elasticsearch::{http::transport::Transport, Elasticsearch, SearchParts};
use serde_json::{json, Value};

#[tokio::main]

async fn main() {
    let transport = Transport::single_node("http://localhost:9200").unwrap();
    let client = Elasticsearch::new(transport);
    let response = client
        .search(SearchParts::Index(&["books"]))
        .from(0)
        .size(3)
        .body(json!({
            "query": {
                "match": {
                    "genre": "Classic"
                }
            }
        }))
        .send()
        .await
        .unwrap();

    println!("{:?}", response.json::<Value>().await.unwrap());
}
