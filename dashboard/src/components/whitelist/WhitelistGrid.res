@react.component
let make = (~accounts: array<string>) => {
  let voteTransactionGrid = Belt.Array.map(accounts, account =>
    <div className="row"> <div className="col gy-3"> <WhitelistCard account /> </div> </div>
  )

  voteTransactionGrid->React.array
}
