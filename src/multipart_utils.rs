use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

use crate::errors::ApiErrors;

// https://users.rust-lang.org/t/file-upload-in-actix-web/64871/2
pub async fn get_multipart_file_path(mut multipart: Multipart) -> Result<String, ApiErrors> {
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

    let random_temp_folder_name = Uuid::new_v4().to_string();
    let random_temp_folder = format!("./{random_temp_folder_name}");
    web::block(|| std::fs::create_dir(random_temp_folder))
        .await
        .map_err(|err| {
            log::warn!("Temp folder could not be created. - {}", err.to_string());
            ApiErrors::UnknownError(String::from("Error trying to create temp folder"))
        })?
        .map_err(|err| {
            log::warn!("Temp folder could not be created. - {}", err.to_string());
            ApiErrors::UnknownError(String::from("Error trying to create temp folder"))
        })?;

    let file_path = format!("./{random_temp_folder_name}/file");
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

    Ok(file_path)
}

pub async fn delete_temp_path(path: &str) {
    let file_path = String::from(path);
    let remove_result = web::block(move || std::fs::remove_dir_all(file_path)).await;
    if let Err(err) = remove_result {
        log::warn!("Temp dir could not be removed. - {}", err.to_string());
    }
}
