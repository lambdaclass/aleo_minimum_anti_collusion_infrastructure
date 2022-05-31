@val @scope(("process", "env")) external _maci_host: option<string> = "REACT_APP_MACI_HOST"
@val @scope(("process", "env")) external _aleo_host: option<string> = "REACT_APP_ALEO_HOST"

let _maci_default_url = "http://127.0.0.1:3000"
let _aleo_default_url = "https://www.aleo.network"

type base_url = {
  aleo: string,
  maci: string,
}

type url = {
  aleo_tx: string => string,
  maci_votes: unit => string,
  maci_whitelist: unit => string,
  maci_results: unit => string,
}

let get_url_or_default = (url: option<string>, default: string): string => {
  switch url {
  | Some(s) => s
  | None => default
  }
}

let base_url = {
  aleo: _aleo_host->get_url_or_default(_aleo_default_url),
  maci: _maci_host->get_url_or_default(_maci_default_url),
}

let url = {
  aleo_tx: (transaction_id: string) => `${base_url.aleo}/txs/${transaction_id}`,
  maci_votes: () => `${base_url.maci}/election/votes`,
  maci_whitelist: () => `${base_url.maci}/election/whitelist`,
  maci_results: () => `${base_url.maci}/election/tally/results`,
}
