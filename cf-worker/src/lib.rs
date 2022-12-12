use worker::*;

use wordle_assistant::recommendation_api::{get_recommendations, State};

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

fn cors_allow_all_origins(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Ok(Response::empty()?.with_status(204))
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    let response = Router::new()
        .get("/", |_, _| Response::from_html(include_str!("index.html")))
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get("/robots.txt", |_, _ctx| {
            Response::ok("User-agent: * \n Disallow: /")
        })
        .get("/openapi.yaml", |_, _| {
            let mut headers = Headers::new();
            headers.set("content-type", "text/yaml")?;
            let txt = include_str!("../../docs/Wordle-Assistant.yaml");
            Ok(Response::from_html(txt)?.with_headers(headers))
        })
        .options("/openapi.yaml", cors_allow_all_origins)
        .post_async("/recommendations", |mut req, _ctx| async move {
            let state = req.json::<State>().await?;
            let recommendations = get_recommendations(state);
            Response::from_json(&recommendations)
        })
        .options("/recommendations", cors_allow_all_origins)
        .run(req, env)
        .await;

    response?.with_cors(
        &Cors::new()
            .with_origins(vec!["*"])
            .with_allowed_headers(vec!["*"])
            .with_methods(Method::all()),
    )
}
