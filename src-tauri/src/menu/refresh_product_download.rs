use crate::{
    application::use_application,
    application_error::{ApplicationError, Result},
    command::get_product_download_path,
    storage::product::Product,
};
use std::fs::read_dir;

pub async fn refresh_product_download() -> Result<()> {
    let download_path = get_product_download_path(use_application().app_handle())?;
    let contents = read_dir(download_path)
        .map_err(|err| ApplicationError::ProductDownloadRefreshError { io_error: err })?;

    Product::remove_all_download()?;

    for entry in contents {
        let entry =
            entry.map_err(|err| ApplicationError::ProductDownloadRefreshError { io_error: err })?;

        if !entry
            .file_type()
            .map_err(|err| ApplicationError::ProductDownloadRefreshError { io_error: err })?
            .is_dir()
        {
            continue;
        }

        let file_name = match entry.file_name().into_string() {
            Ok(file_name) => file_name,
            Err(_) => {
                continue;
            }
        };
        let path = entry.path();
        let path = match path.to_str() {
            Some(path) => path,
            None => {
                continue;
            }
        };

        Product::insert_download(file_name, path)?;
    }

    Ok(())
}
