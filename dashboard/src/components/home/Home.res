@react.component
let make = () => {
  let votes = [5, 2, 7, 0, 0, 0, 2, 4]
  let transaction_ids = [
    "at1cf6aaeq0xzyajupxvp6tvn0v2yzud5y39nukx93cf5vhwrnnnvqqsy5t37",
    "at1nzcdpeqlkzgasa0slr9vypa2ck70xf8qq3lvvnd2lyq3588nxuxqejpuqn",
    "at1l4zsvy00qnc5ztmxewl2c7x2sv28q7nasf55zcah20l5hg66usrqztr8zf",
    "at1xxgt6a69vug2vfelz6p39pvl4dng5emzjtwvpqvmy5t900t9wsxsxlretj",
    "at19jm84srxajprzemqkweer83vmh44lhat6p9vd8sv6q9d7tj06yrqx366yz",
  ]

  <div className="row">
    <div className="col gy-5"> <h4> {"Partial Results"->React.string} </h4> </div>
    <div className="gy-3"> <ResultGrid results={votes} /> </div>
    <div className="col gy-5"> <h4> {"Votes Transactions"->React.string} </h4> </div>
    <div className="gy-3"> <VoteTransactionGrid transaction_ids /> </div>
  </div>
}
