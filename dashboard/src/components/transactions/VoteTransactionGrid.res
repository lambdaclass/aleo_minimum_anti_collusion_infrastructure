@react.component
let make = (~transaction_ids: array<string>) => {
    let voteTransactionGrid =
        Belt.Array.map(transaction_ids, (transaction_id) =>
            <div className="row">
                <div className="col gy-3">
                    <VoteTransactionCard transaction_id={transaction_id}/>
                </div>
            </div>
        )

    voteTransactionGrid -> React.array
}
