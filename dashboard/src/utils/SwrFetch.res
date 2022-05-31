let useSwr = (url: string): Swr.swrResponse<Js.Json.t> => {
  let globalConfig = Swr.SwrConfiguration.useSWRConfig()

  let defaultFetcher = switch globalConfig->Swr.fetcherGet {
  | Some(f) => f
  | None => url => Fetch.fetch(url)->Promise.then(Fetch.Response.json)
  }

  let response = Swr.useSWR_config(url, defaultFetcher, globalConfig)

  response
}

type useSwrGetResponseErrors = Decode

type useSwrGetResponse<'a> =
  | Ready('a)
  | Loading
  | Error(useSwrGetResponseErrors)

let useGet = (url: string, decoder: Js.Json.t => 'a): useSwrGetResponse<'a> => {
  let globalConfig = Swr.SwrConfiguration.useSWRConfig()

  let defaultFetcher = switch globalConfig->Swr.fetcherGet {
  | Some(f) => f
  | None => url => Fetch.fetch(url)->Promise.then(Fetch.Response.json)
  }

  let {data} = Swr.useSWR_config(url, defaultFetcher, globalConfig)

  switch data {
  | Some(jsonData) =>
    switch jsonData->decoder {
    | data => Ready(data)
    | exception _ => Error(Decode)
    }
  | None => Loading
  }
}
