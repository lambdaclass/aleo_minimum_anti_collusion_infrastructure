@react.component
let make = (~accounts: array<string>) => {
  let voteTransactionGrid =
    accounts->Belt.Array.map(account =>
      <div className="row"> <div className="col gy-3"> <WhitelistCard account /> </div> </div>
    )

  voteTransactionGrid->React.array
}
