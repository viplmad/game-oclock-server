use std::{fs::File, io::Write};

use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};

use crate::errors::ApiErrors;

// https://users.rust-lang.org/t/file-upload-in-actix-web/64871/2
pub async fn get_multipart_file(mut multipart: Multipart) -> Result<File, ApiErrors> {
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

    let content_type = field.content_disposition();
    let filename = content_type.get_filename().ok_or_else(|| {
        log::warn!("Filename could not be retrieved from multipart.");
        ApiErrors::UnknownError(String::from("Filename could not be retieved."))
    })?;

    let filepath = format!("./{filename}");
    // Filesystem operations are blocking, use threadpool
    let mut file = web::block(|| std::fs::File::create(filepath))
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

    Ok(file)
}
