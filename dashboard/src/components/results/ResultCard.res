@react.component
let make = (~option: string, ~votes: int) => {
    <div className="card">
        <div className="card-header text-center">
            {option -> React.string}
        </div>
        <div className="card-body">
            <h1 className="text-center">{votes -> React.int}</h1>
        </div>
    </div>
}
