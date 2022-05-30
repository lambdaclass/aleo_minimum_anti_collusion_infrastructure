@react.component
let make = (~option: string, ~votes: int, ~percents: int) => {
  <div className="vote-container card">
    <div className="card-header text-center"> {option->React.string} </div>
    <div className="card-body"> <h1 className="text-center"> {votes->React.int} </h1> </div>
    <div className="card-body">
      <h2 className="percent text-center"> {percents->React.int} {"%"->React.string} </h2>
    </div>
  </div>
}
