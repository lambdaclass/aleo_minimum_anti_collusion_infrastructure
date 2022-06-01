@react.component
let make = () => {
  <nav className="navbar navbar-expand-lg py-4">
    <div className="container-sm">
      <a className="navbar-brand" href="#">
        <span className="strong"> {"Aleo "->React.string} </span> {"MACI"->React.string}
      </a>
      <form className="form-inline my-2 my-lg-0">
        <input
          type_="search" className="search form-control" placeholder="Search" ariaLabel="Search"
        />
        <button className="btn" type_="submit"> <img src="/images/search.svg" /> </button>
      </form>
    </div>
  </nav>
}
