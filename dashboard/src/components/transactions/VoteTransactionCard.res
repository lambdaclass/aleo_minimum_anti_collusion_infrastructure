@react.component
let make = (~transaction_id: string) => {
    <div className="card">
        <div className="card-body">
            <a href={Endpoint.url.aleo_tx(transaction_id)} target="_blank">{transaction_id -> React.string} </a>
        </div>
    </div>
} 
