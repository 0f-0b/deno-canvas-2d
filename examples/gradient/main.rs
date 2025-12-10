use std::sync::Arc;

use deno_core::url::Url;
use deno_core::{JsRuntime, RuntimeOptions, anyhow, v8};
use deno_web::BlobStore;
use tokio::fs;

deno_core::extension!(
    init,
    deps = [canvas_2d],
    esm_entry_point = "ext:init/init.js",
    esm = [dir "examples/gradient", "init.js"],
);

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let blob_store = Arc::new(BlobStore::default());
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![
            deno_webidl::deno_webidl::init(),
            deno_web::deno_web::init(blob_store.clone(), None, Default::default()),
            canvas_2d::canvas_2d::init(),
            init::init(),
        ],
        ..Default::default()
    });
    let url = {
        let promise = runtime
            .execute_script("script.js", include_str!("script.js"))
            .unwrap();
        let future = runtime.resolve(promise);
        let result = runtime
            .with_event_loop_promise(future, Default::default())
            .await?;
        deno_core::scope!(scope, runtime);
        v8::Local::new(scope, result)
            .cast::<v8::String>()
            .to_rust_string_lossy(scope)
    };
    let blob = blob_store
        .get_object_url(Url::parse(&url).unwrap())
        .unwrap();
    let png = blob.read_all().await;
    fs::write("gradient.png", png).await?;
    Ok(())
}
