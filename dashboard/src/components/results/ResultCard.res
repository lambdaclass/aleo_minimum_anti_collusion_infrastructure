@react.component
let make = (~option: string, ~votes: int, ~percents: float) => {
  <div className="vote-container card">
    <div className="card-header text-center"> {option->React.string} </div>
    <div className="card-body"> <h1 className="text-center"> {votes->React.int} </h1> </div>
    <div className="card-body">
      <h2 className="percent text-center">
        {percents->Js.Float.toFixedWithPrecision(~digits=1)->React.string} {"%"->React.string}
      </h2>
    </div>
  </div>
}
