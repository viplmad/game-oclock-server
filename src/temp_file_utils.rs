use actix_web::web;

const BASE_PATH: &str = "./";
const TEMP_DIR_PREFIX: &str = "img_tmp_";

pub async fn delete_temp_dir(directory_path: &str) {
    let dir_path = String::from(directory_path);
    let remove_result = web::block(move || std::fs::remove_dir_all(dir_path)).await;
    if let Err(err) = remove_result {
        log::warn!("Temp directory could not be removed. - {}", err.to_string());
    }
}

pub async fn delete_all_temp_dirs() {
    let dirs_block_result = web::block(|| std::fs::read_dir(BASE_PATH)).await;
    match dirs_block_result {
        Ok(dirs_result) => match dirs_result {
            Ok(dirs) => {
                for dirs in dirs {
                    match dirs {
                        Ok(directory) => {
                            let dir_name = &directory.file_name();
                            let dir_name = dir_name.to_str().unwrap_or_default();
                            if directory.metadata().is_ok_and(|metadata| metadata.is_dir()) && str::starts_with(dir_name, TEMP_DIR_PREFIX) {
                                delete_temp_dir(dir_name).await;
                            }
                        },
                        Err(err) => log::warn!(
                            "Error deleting temp dirs. - Directory from base path could not be read. - {}",
                            err.to_string(),
                        ),
                    }
                }
            }
            Err(err) => log::warn!(
                "Error deleting temp dirs. - Directory from base path could not be retrieved. - {}",
                err.to_string(),
            ),
        },
        Err(err) => log::warn!(
            "Error deleting temp dirs. - Directory from base path could not be retrieved. - {}",
            err.to_string(),
        ),
    };
}

pub fn generate_temp_file_path(temp_dir_path: &str) -> String {
    format!("{temp_dir_path}/file")
}

pub fn generate_temp_dir_path() -> String {
    let temp_dir_name = generate_temp_dir_name();
    format!("{BASE_PATH}{temp_dir_name}")
}

fn generate_temp_dir_name() -> String {
    let random_uuid = crate::uuid_utils::new_random_uuid();
    format!("{TEMP_DIR_PREFIX}{random_uuid}")
}
