@react.component
let make = () => {
    let votes = [5,2,7,0,0,0,2,4]

    <div className="row">
        <div className="col gy-5">
            <h4>{"Partial Results:" -> React.string}</h4>
        </div>
        <div className="gy-3">
            <ResultGrid results={votes}/>
        </div>
        <div className="col gy-5">
            <h4>{"Votes Transactions:" -> React.string}</h4>
        </div>
    </div>
}
