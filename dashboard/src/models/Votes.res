type t = {transactions: array<string>}

let decode = json => {
  open Json.Decode
  {
    transactions: json |> field("transactions", array(string)),
  }
}

let empty = () => {
  {
    transactions: [],
  }
}
