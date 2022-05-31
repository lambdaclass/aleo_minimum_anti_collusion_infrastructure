open SwrFetch

@react.component
let make = (~error: useSwrGetResponseErrors) => {
  let errorType = switch error {
  | Decode => "a format error"
  }

  <div className="alert alert-danger" role="alert">
    {`The data could not be loaded due to ${errorType}.`->React.string}
  </div>
}
