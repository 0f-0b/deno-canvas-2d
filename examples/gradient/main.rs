use std::sync::Arc;

use deno_core::url::Url;
use deno_core::{JsRuntime, RuntimeOptions, anyhow, v8};
use deno_web::{BlobStore, TimersPermission};
use tokio::fs;

struct Permissions;

impl TimersPermission for Permissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

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
            deno_console::deno_console::init(),
            deno_url::deno_url::init(),
            deno_web::deno_web::init::<Permissions>(blob_store.clone(), None),
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
        let scope = &mut runtime.handle_scope();
        let result: v8::Local<v8::String> = v8::Local::new(scope, result).try_into().unwrap();
        result.to_rust_string_lossy(scope)
    };
    let blob = blob_store
        .get_object_url(Url::parse(&url).unwrap())
        .unwrap();
    let png = blob.read_all().await;
    fs::write("gradient.png", png).await?;
    Ok(())
}
