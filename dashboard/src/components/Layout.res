@react.component
let make = (~children: React.element) => {
    <div className="container">
        <div className="row">
            <div className="col">
                <h1>{"Maci for Aleo" -> React.string}</h1>
            </div>
        </div>
        children
    </div>
}
