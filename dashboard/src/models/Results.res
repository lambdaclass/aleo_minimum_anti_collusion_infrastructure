type t = {results: array<int>}

let decode = json => {
  open Json.Decode
  {
    results: json |> field("results", array(int)),
  }
}

let empty = () => {
  {
    results: [],
  }
}
