type t = {accounts: array<string>}

let decode = json => {
  open Json.Decode
  {
    accounts: json |> field("accounts", array(string)),
  }
}

let empty = () => {
  {
    accounts: [],
  }
}
