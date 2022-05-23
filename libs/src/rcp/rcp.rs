use futures::future::join_all;
use serde_json::{json, Value};

//These are nodes we have found that are usually up and answering quickly
const NODES_URLS: [&str; 5] = [
    /*Operators */
    "http://8.210.117.127:3032",
    /*Miners */
    "http://116.202.115.195:3032",
    "http://138.201.224.221:3032",
    "http://206.189.97.241:3032",
    /*Clients */
    "http://142.132.146.253:3032",
];

///Sends the transactions synchronically to many known nodes
///and returns all the results in an array of jsons
pub fn sync_spray_transaction(transaction_hex_data: String) -> Vec<Result<Value, reqwest::Error>> {
    let request_json = json!({
        "jsonrpc": "2.0",
        "id": "1",
        "method": "sendtransaction",
        "params": [
            transaction_hex_data
        ]
    });

    let client = reqwest::blocking::Client::new();
    //TO DO: Use our own Aleo node client, or pass it as an argument

    let mut requests_results = vec![];
    for node in NODES_URLS {
        let send_result = client.post(node).json(&request_json).send();
        let v = match send_result {
            Ok(send_result) => {
                let with_status_error = send_result.error_for_status();
                match with_status_error {
                    Ok(msg) => msg.json(),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        };
        requests_results.push(v);
    }

    requests_results
}

/// Gets transaction first public record
//TO DO: Get all public records
pub async fn get_transaction_public_data(transaction_id: String) -> Result<String, reqwest::Error> {
    let request_json = json!({
        "jsonrpc": "2.0",
        "id": "2",
        "method": "gettransaction",
        "params": [
            transaction_id
        ]
    });

    let client = reqwest::Client::new();
    //TO DO: Check if node is alive before sending the request
    let send_result = client
        .post(NODES_URLS[0])
        .json(&request_json)
        .send()
        .await
        .unwrap();

    let res_json_result: Result<Value, reqwest::Error> = send_result.json().await;
    Ok(
        res_json_result.unwrap()["result"]["decrypted_records"][0]["payload"]
            .as_str()
            .unwrap()
            .to_string(),
    )
}

///Converts the public data string of a record to a vote string
pub fn public_data_to_vote(data: String) -> String {
    //We use the first 2 elements of the string to represent the number
    let sliced_string = data[0..2].to_string();
    let sliced_str: &str = sliced_string.as_str();
    let u32_vote = u32::from_str_radix(sliced_str, 16).unwrap();
    u32_vote.to_string()
}

pub async fn get_transactions_public_data(
    transactions_id: Vec<String>,
) -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();

    let transactions_futures = transactions_id.iter().map(|transaction_id| {
        let client = &client;

        let request_json = json!({
            "jsonrpc": "2.0",
            "id": "2",
            "method": "gettransaction",
            "params": [
                transaction_id
            ]
        });

        async move {
            let resp = client
                .post(NODES_URLS[0])
                .json(&request_json)
                .send()
                .await
                .unwrap();

            let resp_json: Result<Value, reqwest::Error> = resp.json().await;

            resp_json
        }
    });

    let transactions_results = join_all(transactions_futures).await;

    let votes: Vec<String> = transactions_results
        .into_iter()
        .map(|result| {
            let res_json = result.unwrap();
            res_json["result"]["decrypted_records"][0]["payload"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect();

    Ok(votes)
}
