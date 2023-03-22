mod usecases;
mod infrastructures;
mod models;
use tauri::Manager;

fn main() {
    use tauri::async_runtime::block_on;
    let sqlite_pool = block_on(infrastructures::db::create_sqlite_pool()).unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            usecases::content::get,
            usecases::content::save
        ])
        // ハンドラからコネクションプールにアクセスできるよう、登録する
        .setup(|app| {
            app.manage(sqlite_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
