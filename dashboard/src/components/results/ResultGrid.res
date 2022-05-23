@react.component
let make = (~results: array<int>) => {

    let resultsGrid = Belt.Array.mapWithIndex(results, (i, result) =>
        <div className="col">
            <ResultBox option={`Option ${i -> Belt.Int.toString}`} votes={result}/>
        </div>
    )

    <div className="row">
        {resultsGrid -> React.array}
    </div>
}
