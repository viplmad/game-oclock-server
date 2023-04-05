use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};

use crate::errors::ApiErrors;
use crate::models::FileTempPath;

const TEMP_DIR_PREFIX: &str = "img_tmp_";

// https://users.rust-lang.org/t/file-upload-in-actix-web/64871/2
pub async fn get_multipart_file_path(mut multipart: Multipart) -> Result<FileTempPath, ApiErrors> {
    let mut field = match multipart.try_next().await {
        Ok(optional_field) => match optional_field {
            Some(field) => Ok(field),
            None => Err(ApiErrors::UnknownError(String::from(
                "File could not be parsed.",
            ))),
        },
        Err(err) => {
            log::warn!("File could not be parsed. - {}", err.to_string());
            return Err(ApiErrors::UnknownError(String::from(
                "File could not be parsed.",
            )));
        }
    }?;

    let temp_folder = build_temp_dir_name();
    let directory_path = format!("./{temp_folder}");
    let directory_path_copy = directory_path.clone();

    web::block(|| std::fs::create_dir(directory_path_copy))
        .await
        .map_err(|err| {
            log::warn!("Temp folder could not be created. - {}", err.to_string());
            ApiErrors::UnknownError(String::from("Error trying to create temp folder"))
        })?
        .map_err(|err| {
            log::warn!("Temp folder could not be created. - {}", err.to_string());
            ApiErrors::UnknownError(String::from("Error trying to create temp folder"))
        })?;

    let file_path = format!("./{temp_folder}/file");
    let file_path_copy = file_path.clone();

    // Filesystem operations are blocking, use threadpool
    let mut file = web::block(|| std::fs::File::create(file_path_copy))
        .await
        .map_err(|err| {
            log::warn!(
                "File could not be created from multipart. - {}",
                err.to_string()
            );
            ApiErrors::UnknownError(String::from("Error trying to create file"))
        })?
        .map_err(|err| {
            log::warn!(
                "File could not be created from multipart. - {}",
                err.to_string()
            );
            ApiErrors::UnknownError(String::from("Error trying to create file"))
        })?;

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // Filesystem operations are blocking, use threadpool
        file = web::block(move || file.write_all(&data).map(|_| file))
            .await
            .map_err(|err| {
                log::warn!(
                    "File could not be created from multipart. - {}",
                    err.to_string()
                );
                ApiErrors::UnknownError(String::from("Error trying to create file"))
            })?
            .map_err(|err| {
                log::warn!(
                    "File could not be created from multipart. - {}",
                    err.to_string()
                );
                ApiErrors::UnknownError(String::from("Error trying to create file"))
                // TODO Use other error -> this is for service errors
            })?;
    }

    Ok(FileTempPath {
        directory_path,
        file_path,
    })
}

pub async fn delete_temp_path(directory_path: &str) {
    let dir_path = String::from(directory_path);
    let remove_result = web::block(move || std::fs::remove_dir_all(dir_path)).await;
    if let Err(err) = remove_result {
        log::warn!("Temp dir could not be removed. - {}", err.to_string());
    }
}

fn build_temp_dir_name() -> String {
    let random_uuid = crate::uuid_utils::new_random_uuid();
    format!("{TEMP_DIR_PREFIX}{random_uuid}")
}
