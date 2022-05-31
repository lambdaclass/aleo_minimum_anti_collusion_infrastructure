@react.component
let make = (~account: string) => {
  <div className="transaction-id-container card">
    <div className="card-body">
      <h3 className="card-label"> {"Account"->React.string} </h3>
      <a className="link" href={Endpoint.url.aleo_tx(account)} target="_blank">
        {account->React.string}
      </a>
    </div>
  </div>
}
