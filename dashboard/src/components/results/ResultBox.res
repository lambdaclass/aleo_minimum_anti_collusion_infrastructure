@react.component
let make = (~option: string, ~votes: int) => {
    <div className="card">
        <div className="card-body">
            <h5 className="card-title text-center">{option -> React.string}</h5>
            <h1 className="text-center">{votes -> React.int}</h1>
        </div>
    </div>
}
