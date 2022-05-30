@react.component
let make = (~results: array<int>) => {
  let resultsGrid = Belt.Array.mapWithIndex(results, (i, result) =>
    <div className="col">
      <ResultCard option={`Option ${i->Belt.Int.toString}`} votes={result} percents={result / 2} />
    </div>
  )

  <div className="row"> {resultsGrid->React.array} </div>
}
