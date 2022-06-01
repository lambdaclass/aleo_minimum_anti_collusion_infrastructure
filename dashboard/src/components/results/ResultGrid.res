@react.component
let make = (~results: array<int>) => {
  let totalVotes = results->Belt.Array.reduce(0, (x, i) => x + i)->Belt.Float.fromInt

  let resultsGrid =
    results->Belt.Array.mapWithIndex((i, result) =>
      <div className="col gy-3">
        <ResultCard
          option={`Option ${(i + 1)->Belt.Int.toString}`}
          votes={result}
          percents={result->Belt.Float.fromInt /. totalVotes *. 100.0}
        />
      </div>
    )

  <div className="row"> {resultsGrid->React.array} </div>
}
