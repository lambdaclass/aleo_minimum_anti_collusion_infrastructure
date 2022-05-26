let useGet = (url: string) => {
    let globalConfig = Swr.SwrConfiguration.useSWRConfig()

    let defaultFetcher = switch globalConfig -> Swr.fetcherGet {
        | Some(f) => f
        | None => (url) => Fetch.fetch(url) -> Promise.then(Fetch.Response.json)
    }

    let response = Swr.useSWR_config(
        url,
        defaultFetcher,
        globalConfig
    )

    response
}
