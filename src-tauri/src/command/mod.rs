use crate::{application_error::Result, storage::setting::Setting};
use std::path::PathBuf;
use tauri::{api::path::download_dir, generate_handler, Builder, Runtime};

mod account_management;
mod latest_product_query;
mod product;
mod setting;
mod window;

pub trait CommandProvider<R>
where
    R: Runtime,
{
    fn attach_commands(self) -> Self;
}

impl<R> CommandProvider<R> for Builder<R>
where
    R: Runtime,
{
    fn attach_commands(self) -> Self {
        self.invoke_handler(generate_handler![
            account_management::account_management_list_accounts,
            account_management::account_management_get_account,
            account_management::account_management_add_account,
            account_management::account_management_update_account,
            account_management::account_management_remove_account,
            account_management::account_management_test_account,
            latest_product_query::latest_product_query_get,
            latest_product_query::latest_product_query_set,
            product::product_list_products,
            product::product_download_product,
            product::product_open_downloaded_folder,
            product::product_remove_downloaded_product,
            setting::setting_get,
            setting::display_language_setting_get,
            setting::setting_browse_default_root_directory,
            setting::setting_close,
            setting::setting_save_and_close,
            window::show_window,
            window::spawn_window_account_add,
            window::spawn_window_account_edit,
        ])
    }
}

pub fn get_product_download_path<R: Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<PathBuf> {
    let setting = Setting::get()?;
    Ok(setting.download_root_dir.unwrap_or_else(|| {
        download_dir()
            .unwrap_or_else(|| app_handle.path_resolver().app_local_data_dir().unwrap())
            .join("DLsite")
    }))
}
