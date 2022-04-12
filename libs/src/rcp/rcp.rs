use serde_json::{json, Value};

//These are nodes we have found that are usually up and answering quickly
const NODES_URLS: [&str; 6] = [
    /*Operators */
    "http://8.210.117.127:3032",
    /*Miners */
    "http://116.202.115.195:3032",
    "http://138.201.224.221:3032",
    "http://206.189.97.241:3032",
    "http://71.187.18.245:3032",
    /*Clients */
    "http://142.132.146.253:3032",
];

///Sends the transactions synchronically to many known nodes
///and returns all the results in an array of jsons
pub fn sync_spray_transaction(transaction_hex_data: String) -> Vec<Value> {
    //TO DO: Move to a shared library
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

        match send_result {
            Ok(send_result) => {
                let res_json_result: Result<Value, reqwest::Error> = send_result.json();
                requests_results.push(match res_json_result {
                    Ok(res) => res,
                    Err(error) => json!({"error":error.to_string()}),
                });
            }
            Err(e) => {
                requests_results.push(json!({"error":e.to_string()}));
            }
        };
    }

    return requests_results;
}
