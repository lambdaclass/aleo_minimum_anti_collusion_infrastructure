@react.component
let make = () => {
    let url = RescriptReactRouter.useUrl()
    <>
        <Navbar/>
        <Layout>
            {switch url.path {
                | list{} => <Home/>
                | _ => <PageNotFound/>
            }}
        </Layout>
    </>
}
