@react.component
let make = () => {
    <nav className="navbar navbar-expand-lg navbar-light bg-light">
        <div className="container-sm">
            <a className="navbar-brand" href="#">{"Maci for Aleo" -> React.string}</a>
            {React.cloneElement(
                <button className="navbar-toggler" type_="button" ariaControls="navbarNavAltMarkup" ariaExpanded={false} ariaLabel="Toggle navigation"/>,
                {"data-bs-toggle":"collapse", "data-bs-target":"#navbarNavAltMarkup"}
            )}
            <span className="navbar-toggler-icon"></span>
            <div className="collapse navbar-collapse" id="navbarNavAltMarkup">
            <div className="navbar-nav">
            </div>
            </div>
        </div>
    </nav>
}
