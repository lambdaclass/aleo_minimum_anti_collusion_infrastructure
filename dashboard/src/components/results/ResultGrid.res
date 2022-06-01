@react.component
let make = (~results: array<int>) => {
  let resultsGrid =
    results->Belt.Array.mapWithIndex((i, result) =>
      <div className="col gy-3">
        <ResultCard
          option={`Option ${i->Belt.Int.toString}`} votes={result} percents={result / 2}
        />
      </div>
    )

  <div className="row"> {resultsGrid->React.array} </div>
}
