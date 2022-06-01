@react.component
let make = (~transaction: array<string>) => {
  let voteTransactionGrid =
    transaction->Belt.Array.map(transaction_id =>
      <div className="row">
        <div className="col gy-3"> <VoteTransactionCard transaction_id={transaction_id} /> </div>
      </div>
    )

  voteTransactionGrid->React.array
}
