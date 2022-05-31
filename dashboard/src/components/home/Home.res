@react.component
let make = () => {
  let results = Endpoint.url.maci_results()->SwrFetch.useGet(Results.decode)
  let votes = Endpoint.url.maci_votes()->SwrFetch.useGet(Votes.decode)

  let resultsElement = switch results {
  | Ready(data) => <ResultGrid results={data.results} />
  | Loading => <SwrLoading />
  | Error(error) => <SwrError error />
  }

  let transactionElement = switch votes {
  | Ready(data) => <VoteTransactionGrid transaction={data.transactions} />
  | Loading => <SwrLoading />
  | Error(error) => <SwrError error />
  }

  <div className="row">
    <div className="col gy-5"> <h4> {"Partial Results"->React.string} </h4> </div>
    <div className="gy-3"> {resultsElement} </div>
    <div className="col gy-5"> <h4> {"Votes Transactions"->React.string} </h4> </div>
    {transactionElement}
  </div>
}
