use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};

use crate::errors::ApiErrors;
use crate::models::FileTempPath;
use crate::temp_file_utils;

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

    let directory_path = temp_file_utils::generate_temp_dir_path();
    let directory_path_copy = directory_path.clone();

    web::block(|| std::fs::create_dir(directory_path_copy))
        .await
        .map_err(|err| {
            log::warn!("Temp directory could not be created. - {}", err.to_string());
            ApiErrors::UnknownError(String::from("Error trying to create temp directory."))
        })?
        .map_err(|err| {
            log::warn!("Temp directory could not be created. - {}", err.to_string());
            ApiErrors::UnknownError(String::from("Error trying to create temp directory."))
        })?;

    let file_path = temp_file_utils::generate_temp_file_path(&directory_path);
    let file_path_copy = file_path.clone();

    // Filesystem operations are blocking, use threadpool
    let mut file = web::block(|| std::fs::File::create(file_path_copy))
        .await
        .map_err(|err| {
            log::warn!(
                "File could not be created from multipart. - {}",
                err.to_string()
            );
            ApiErrors::UnknownError(String::from("Error trying to create file."))
        })?
        .map_err(|err| {
            log::warn!(
                "File could not be created from multipart. - {}",
                err.to_string()
            );
            ApiErrors::UnknownError(String::from("Error trying to create file."))
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
                ApiErrors::UnknownError(String::from("Error trying to create file."))
            })?
            .map_err(|err| {
                log::warn!(
                    "File could not be created from multipart. - {}",
                    err.to_string()
                );
                ApiErrors::UnknownError(String::from("Error trying to create file."))
            })?;
    }

    Ok(FileTempPath {
        directory_path,
        file_path,
    })
}
