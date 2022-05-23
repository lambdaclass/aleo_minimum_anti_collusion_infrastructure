@react.component
let make = () => {
    let url = RescriptReactRouter.useUrl()

    <Layout>
        {switch url.path {
            | list{} => <Home/>
            | _ => <PageNotFound/>
        }}
    </Layout>
}
