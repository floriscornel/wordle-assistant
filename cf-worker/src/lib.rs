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

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    Router::new()
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
            Ok(Response::ok(txt)?.with_headers(headers))
        })
        .post_async("/recommendations", |mut req, _ctx| async move {
            let state = req.json::<State>().await?;
            let recommendations = get_recommendations(state);
            let cors = Cors::new().with_origins(vec!["*"]).with_methods(vec![
                Method::Get,
                Method::Post,
                Method::Options,
            ]);
            Response::from_json(&recommendations)?.with_cors(&cors)
        })
        .run(req, env)
        .await
}
