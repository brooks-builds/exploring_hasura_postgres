use std::env;

use graphql_client::{GraphQLQuery, Response};

#[allow(non_camel_case_types)]
type money = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "queries/list_stores.graphql",
    response_derives = "Debug"
)]
struct ListStores;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "queries/list_all_items.graphql",
    response_derives = "Debug"
)]
struct ListAllItems;

#[derive(Debug)]
struct Item {
    id: i64,
    name: String,
    price: String,
    store_name: String,
}

impl Item {
    pub fn new(id: i64, name: String, price: String, store_name: String) -> Self {
        Self {
            id,
            name,
            price,
            store_name,
        }
    }
}

async fn get_stores(uri: String, key: String) -> Result<(), Box<dyn std::error::Error>> {
    let variables = list_stores::Variables;
    let request_body = ListStores::build_query(variables);
    let client = reqwest::Client::new();
    let result = client
        .post(&uri)
        .header("x-hasura-admin-secret", &key)
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<list_stores::ResponseData> = result.json().await?;
    // dbg!(response_body);
    Ok(())
}

async fn get_items(uri: &str, key: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let result: Response<list_all_items::ResponseData> = reqwest::Client::new()
        .post(uri)
        .header("x-hasura-admin-secret", key)
        .json(&ListAllItems::build_query(list_all_items::Variables))
        .send()
        .await?
        .json()
        .await?;
    let items = result
        .data
        .unwrap()
        .items
        .iter()
        .map(|graphql_item| {
            Item::new(
                graphql_item.id,
                graphql_item.name.clone().unwrap(),
                graphql_item.price.clone().unwrap(),
                graphql_item.store.as_ref().unwrap().name.clone().unwrap(),
            )
        })
        .collect();
    Ok(items)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "mutations/create_store.graphql",
    response_derives = "Debug"
)]
struct CreateStore;

async fn create_store_fn(
    uri: &str,
    key: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let variables = create_store::Variables {
        name: name.to_owned(),
    };
    let result: Response<create_store::ResponseData> = reqwest::Client::new()
        .post(uri)
        .header("x-hasura-admin-secret", key)
        .json(&CreateStore::build_query(variables))
        .send()
        .await?
        .json()
        .await?;
    dbg!(result);
    Ok(())
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "mutations/update_item_store.graphql",
    response_derives = "Debug"
)]
struct UpdateStore;

async fn update_item_store(
    uri: &str,
    key: &str,
    item_id: i64,
    new_store_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let variables = update_store::Variables {
        id: update_store::items_pk_columns_input { id: item_id },
        store_id: new_store_id,
    };
    let result: Response<update_store::ResponseData> = reqwest::Client::new()
        .post(uri)
        .header("x-hasura-admin-secret", key)
        .json(&UpdateStore::build_query(variables))
        .send()
        .await?
        .json()
        .await?;
    dbg!(result);
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let graphql_uri = env::var("GRAPHQL_URI").unwrap();
    let graphql_key = env::var("GRAPHQL_KEY").unwrap();
    // get_stores(graphql_uri.clone(), graphql_key.clone())
    //     .await
    //     .unwrap();
    // let items = get_items(&graphql_uri, &graphql_key).await.unwrap();
    // dbg!(items);
    // create_store_fn(&graphql_uri, &graphql_key, "Home Depot")
    //     .await
    //     .unwrap();
    update_item_store(&graphql_uri, &graphql_key, 1, 6)
        .await
        .unwrap();
}
