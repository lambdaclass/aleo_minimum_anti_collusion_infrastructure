type base_url = {
    aleo: string
}

type url = {
    aleo_tx: string => string
}

let base_url = {
    aleo: "https://www.aleo.network"
}

let url = {
    aleo_tx: (transaction_id: string) => `${base_url.aleo}/txs/${transaction_id}`
}
