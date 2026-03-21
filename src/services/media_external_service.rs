use crate::errors::ApiErrors;
use crate::models::{ExternalMediaIdDTO, MediaRawDTO, MediaType, NewManualMediaDTO};

use chrono::DateTime;
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone)]
pub struct MediaExternalService {
    pub igdb_client: IgdbClient,
}

impl MediaExternalService {
    pub fn with(igdb_client: IgdbClient) -> Self {
        Self { igdb_client }
    }
}

impl MediaExternalService {
    pub async fn get(&self, id: &ExternalMediaIdDTO) -> Result<NewManualMediaDTO, ApiErrors> {
        if id.source.to_lowercase() == IGDB {
            return self.igdb_client.get(&id.id).await;
        }
        return Err(ApiErrors::NotSupported(format!(
            "Unsupported media source '{}'",
            id.source
        )));
    }

    pub async fn search(
        &self,
        source: &str,
        query: &str,
        size: u64,
    ) -> Result<Vec<(ExternalMediaIdDTO, MediaRawDTO)>, ApiErrors> {
        if source.to_lowercase() == IGDB {
            return self.igdb_client.search(query, size).await;
        }
        return Err(ApiErrors::NotSupported(format!(
            "Unsupported media source '{}'",
            source
        )));
    }
}

#[derive(Clone)]
pub struct IgdbClient {
    client: Client,
    client_id: String,
    client_secret: String,
}

const IGDB: &str = "igdb";

const AUTH_BASE_URL: &str = "https://id.twitch.tv/oauth2";
const AUTH_PATH: &str = "/token";

const IGDB_BASE_URL: &str = "https://api.igdb.com/v4";
const IGDB_GAMES_PATH: &str = "/games";

impl IgdbClient {
    pub async fn get(&self, id: &str) -> Result<NewManualMediaDTO, ApiErrors> {
        let access_token = self.auth().await?;

        let fields = Self::get_fields();
        let body = format!("where id = {};fields {};", id, fields.join(", "));

        let resp = self
            .client
            .post(format!("{}{}", IGDB_BASE_URL, IGDB_GAMES_PATH))
            .header(String::from("Client-ID"), self.client_id.clone())
            .bearer_auth(access_token)
            .body(body)
            .send()
            .await
            .unwrap() // TODO
            .json::<Vec<IgdbGamesResponse>>()
            .await
            .unwrap()
            .remove(0); // TODO

        Ok(NewManualMediaDTO {
            kind: Some(Self::get_kind(resp.game_type)),
            title: Some(resp.name),
            edition: resp.version_title,
            release_date: resp
                .first_release_date
                .map(|v| DateTime::from_timestamp_secs(v).unwrap()),
            genres: resp.genres.into_iter().map(|e| e.name).collect(),
            series: resp.collections.into_iter().map(|e| e.name).collect(),
            image_url: Some(resp.cover.url),
            parent_id: None,
            parent_order: None,
        })
    }

    pub async fn search(
        &self,
        query: &str,
        size: u64,
    ) -> Result<Vec<(ExternalMediaIdDTO, MediaRawDTO)>, ApiErrors> {
        let access_token = self.auth().await?;

        let fields = Self::get_fields();
        let body = format!(
            "search \"{}\";fields {};limit {};",
            query,
            fields.join(", "),
            size
        );

        let resp = self
            .client
            .post(format!("{}{}", IGDB_BASE_URL, IGDB_GAMES_PATH))
            .header(String::from("Client-ID"), self.client_id.clone())
            .bearer_auth(access_token)
            .body(body)
            .send()
            .await
            .unwrap() // TODO
            .json::<Vec<IgdbGamesResponse>>()
            .await
            .unwrap(); // TODO

        Ok(resp
            .into_iter()
            .map(|item| {
                (
                    ExternalMediaIdDTO {
                        source: String::from(IGDB),
                        id: item.id.to_string(),
                    },
                    MediaRawDTO {
                        id: Uuid::default(), // TODO
                        kind: Self::get_kind(item.game_type),
                        title: item.name,
                        edition: item.version_title.unwrap_or_default(),
                        release_date: item
                            .first_release_date
                            .map(|v| DateTime::from_timestamp_secs(v).unwrap()),
                        genres: item.genres.into_iter().map(|e| e.name).collect(),
                        series: item.collections.into_iter().map(|e| e.name).collect(),
                        image_url: Some(item.cover.url),
                        parent_id: None,
                        parent_order: None,
                        added_datetime: crate::date_utils::now(), // TODO
                        updated_datetime: crate::date_utils::now(), // TODO
                    },
                )
            })
            .collect())
    }

    fn get_fields() -> Vec<&'static str> {
        vec![
            "name",
            "version_title",
            "cover.url",
            "first_release_date",
            "genres.name",
            "collections.name",
            "game_type.type",
            "parent_game",
        ]
    }

    fn get_kind(game_type: Option<i16>) -> MediaType {
        if let Some(t) = game_type
            && (t == 1 || t == 2 || t == 4 || t == 13 || t == 14)
        {
            return MediaType::GameDlc;
        }
        MediaType::Game
    }

    async fn auth(&self) -> Result<String, ApiErrors> {
        let grant_type = "client_credentials";
        let auth_resp = self
            .client
            .post(format!(
                "{}{}?client_id={}&client_secret={}&grant_type={}",
                AUTH_BASE_URL, AUTH_PATH, &self.client_id, &self.client_secret, grant_type
            ))
            .send()
            .await
            .unwrap() // TODO
            .json::<TwitchAuthResponse>()
            .await
            .unwrap();

        Ok(auth_resp.access_token)
    }
}

#[derive(Deserialize)]
struct TwitchAuthResponse {
    access_token: String,
    // expires_in: u64,
    // token_type: String,
}

#[derive(Deserialize)]
struct IgdbGamesResponse {
    id: i64,
    name: String,
    version_title: Option<String>,
    cover: IgdbCoverResponse,
    first_release_date: Option<i64>,
    genres: Vec<IgdbElementResponse>,
    collections: Vec<IgdbElementResponse>,
    game_type: Option<i16>,
    // parent_game: Option<i64>,
}
#[derive(Deserialize)]
struct IgdbCoverResponse {
    url: String,
}
#[derive(Deserialize)]
struct IgdbElementResponse {
    name: String,
}

/// Connection options to Sqlx Postgres.
#[derive(Debug)]
pub struct IgdbClientPoolBuilder;

impl IgdbClientPoolBuilder {
    pub async fn from_env() -> Result<IgdbClient, reqwest::Error> {
        let client_id = std::env::var("IGDB_CLIENT_ID")
            .expect("IGDB client id not set. Set through 'IGDB_CLIENT_ID' environemnt variable.");
        let client_secret = std::env::var("IGDB_CLIENT_SECRET").expect(
            "IGDB client secret not set. Set through 'IGDB_CLIENT_SECRET' environemnt variable.",
        );

        // Manually-constructed options
        let client = Client::builder().build().map(|res| {
            log::info!("Igdb client set with client id {}", client_id,);
            res
        })?;

        Ok(IgdbClient {
            client,
            client_id,
            client_secret,
        })
    }
}
