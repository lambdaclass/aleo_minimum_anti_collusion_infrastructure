@react.component
let make = (~msg: string) => {
  <div className="row">
    <div className="col gy-3">
      <div className="alert alert-dark vote-container" role="alert"> {msg->React.string} </div>
    </div>
  </div>
}
