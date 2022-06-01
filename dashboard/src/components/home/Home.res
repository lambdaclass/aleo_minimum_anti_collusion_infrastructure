@react.component
let make = () => {
  let whitelist = Endpoint.url.maci_whitelist()->SwrFetch.useGet(WhiteList.decode)
  let results = Endpoint.url.maci_results()->SwrFetch.useGet(Results.decode)
  let votes = Endpoint.url.maci_votes()->SwrFetch.useGet(Votes.decode)

  let resultsElement = switch results {
  | Ready({results: []}) => <EmptyAlert msg="There are no results available to display yet." />
  | Ready(data) => <ResultGrid results={data.results} />
  | Loading => <SwrLoading />
  | Error(error) => <SwrError error />
  }

  let votesElement = switch votes {
  | Ready({transactions: []}) => <EmptyAlert msg="No one has voted yet." />
  | Ready(data) => <VoteTransactionGrid transaction={data.transactions} />
  | Loading => <SwrLoading />
  | Error(error) => <SwrError error />
  }

  let whitelistElement = switch whitelist {
  | Ready({accounts: []}) => <EmptyAlert msg="The whitelist has not been generated yet." />
  | Ready(data) => <WhitelistGrid accounts={data.accounts} />
  | Loading => <SwrLoading />
  | Error(error) => <SwrError error />
  }

  <>
    <div className="row">
      <div className="col gy-3"> <h4> {"Tally"->React.string} </h4> </div>
    </div>
    <div className="row"> <div className="col"> {resultsElement} </div> </div>
    <div className="row">
      <div className="col gy-5"> <h4> {"Votes"->React.string} </h4> </div>
    </div>
    <div className="row"> <div className="col"> {votesElement} </div> </div>
    <div className="row">
      <div className="col gy-5"> <h4> {"Whitelisted Accounts"->React.string} </h4> </div>
    </div>
    <div className="row"> <div className="col"> {whitelistElement} </div> </div>
  </>
}
