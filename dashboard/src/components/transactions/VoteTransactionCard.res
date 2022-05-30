@react.component
let make = (~transaction_id: string) => {
  <div className="transaction-id-container card">
    <div className="card-body">
      <h3 className="card-label"> {"Transaction ID"->React.string} </h3>
      <a className="link" href={Endpoint.url.aleo_tx(transaction_id)} target="_blank">
        {transaction_id->React.string}
      </a>
    </div>
  </div>
}
