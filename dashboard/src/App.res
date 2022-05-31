@react.component
let make = () => {
  let url = RescriptReactRouter.useUrl()

  let swrValue = Swr.swrConfiguration(
    ~refreshInterval=0,
    ~fetcher=url => Fetch.fetch(url)->Promise.then(Fetch.Response.json),
    (),
  )

  <Swr.SwrConfigProvider value={swrValue}>
    <Navbar />
    <Layout>
      {switch url.path {
      | list{} => <Home />
      | _ => <PageNotFound />
      }}
    </Layout>
  </Swr.SwrConfigProvider>
}
