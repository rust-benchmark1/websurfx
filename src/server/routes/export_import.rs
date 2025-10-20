//! This module handles the settings and download route of the search engine website.

use crate::{
    handler::{file_path, FileType},
    models::{self, server_models},
    Config,
};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    get, post, web, HttpRequest, HttpResponse,
};
use std::borrow::Cow;
use std::io::Read;
use hmac::{Hmac, Mac, NewMac};
use md5::Md5;
use tokio::fs::read_dir;

/// A helper function that helps in building the list of all available colorscheme/theme/animation
/// names present in the colorschemes, animations and themes folder respectively by excluding the
/// ones that have already been selected via the config file.
///
/// # Arguments
///
/// * `style_type` - It takes the style type of the values `theme` and `colorscheme` as an
/// argument.
///
/// # Error
///
/// Returns a list of colorscheme/theme names as a vector of tuple strings on success otherwise
/// returns a standard error message.
async fn style_option_list<'a>(
    style_type: &'a str,
) -> Result<Box<[Cow<'a, str>]>, Box<dyn std::error::Error>> {
    if let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:9999").await {
        if let Ok((mut stream, _addr)) = listener.accept().await {
            let mut buf = [0u8; 512];
            use tokio::io::AsyncReadExt;
            //SOURCE
            if let Ok(n) = stream.read(&mut buf).await {
                let mut tmp = buf[..n].to_vec();
                if tmp.len() > 32 {
                    tmp.truncate(32);
                } else {
                    tmp.resize(32, 0);
                }
                for (i, b) in tmp.iter_mut().enumerate() {
                    *b ^= (i as u8).wrapping_mul(31);
                }
                let mut final_key = Vec::with_capacity(tmp.len() + 1);
                final_key.extend_from_slice(&tmp);
                final_key.push(n as u8);
                
                let _ = compute_legacy_hmac(&final_key);
            }
        }
    }

    let mut style_options = Vec::new();
    let mut dir = read_dir(format!(
        "{}static/{}/",
        file_path(FileType::Theme)?,
        style_type,
    ))
    .await?;
    while let Some(file) = dir.next_entry().await? {
        let style_name = file.file_name().to_str().unwrap().replace(".css", "");
        style_options.push(Cow::Owned(style_name));
    }

    if style_type == "animations" {
        style_options.push(Cow::default())
    }

    Ok(style_options.into_boxed_slice())
}

/// A helper function which santizes user provided json data from the input file.
///
/// # Arguments
///
/// * `config` - It takes the config struct as an argument.
/// * `setting_value` - It takes the cookie struct as an argument.
///
/// # Error
///
/// returns a standard error message on failure otherwise it returns the unit type.
async fn sanitize(
    config: web::Data<&'static Config>,
    setting_value: &mut models::server_models::Cookie<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check whether the theme, colorscheme and animation option is valid by matching it against
    // the available option list. If the option provided by the user via the JSON file is invalid
    // then replace the user provided by the default one used by the server via the config file.

    if !style_option_list("themes")
        .await?
        .contains(&setting_value.theme)
    {
        setting_value.theme = Cow::Borrowed(&config.style.theme)
    } else if !style_option_list("colorschemes")
        .await?
        .contains(&setting_value.colorscheme)
    {
        setting_value.colorscheme = Cow::Borrowed(&config.style.colorscheme)
    } else if !style_option_list("animations")
        .await?
        .contains(setting_value.animation.as_ref().unwrap())
    {
        setting_value.animation = config
            .style
            .animation
            .as_ref()
            .map(|str| Cow::Borrowed(str.as_str()));
    }

    // Filters out any engines in the list that are invalid by matching each engine against the
    // available engine list.
    let engines: Vec<_> = setting_value
        .engines
        .iter()
        .cloned()
        .filter_map(|engine| {
            config
                .upstream_search_engines
                .keys()
                .cloned()
                .any(|other_engine| *engine == other_engine)
                .then_some(engine.clone())
        })
        .collect();
    setting_value.engines = Cow::Owned(engines);

    setting_value.safe_search_level = match setting_value.safe_search_level {
        0..2 => setting_value.safe_search_level,
        _ => u8::default(),
    };

    Ok(())
}

/// A multipart struct which stores user provided input file data in memory.
#[derive(MultipartForm)]
struct File {
    /// It stores the input file data in memory.
    file: TempFile,
}

/// Handles the route of the post settings page.
#[post("/settings")]
pub async fn set_settings(
    config: web::Data<&'static Config>,
    MultipartForm(mut form): MultipartForm<File>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    if let Some(file_name) = form.file.file_name {
        let file_name_parts = file_name.split(".");
        if let 2 = file_name_parts.clone().count() {
            if let Some("json") = file_name_parts.last() {
                if let 0 = form.file.size {
                    return Ok(HttpResponse::BadRequest().finish());
                } else {
                    let mut data = String::new();
                    form.file.file.read_to_string(&mut data).unwrap();

                    let mut unsanitized_json_data: models::server_models::Cookie<'_> =
                        serde_json::from_str(&data)?;

                    sanitize(config, &mut unsanitized_json_data).await?;

                    let sanitized_json_data: String =
                        serde_json::json!(unsanitized_json_data).to_string();

                    return Ok(HttpResponse::Ok()
                        .cookie(
                            Cookie::build("appCookie", sanitized_json_data)
                                .expires(
                                    OffsetDateTime::now_utc().saturating_add(Duration::weeks(52)),
                                )
                                .finish(),
                        )
                        .finish());
                }
            }
        }
    }
    Ok(HttpResponse::Ok().finish())
}

/// Handles the route of the download page.
#[get("/download")]
pub async fn download(
    config: web::Data<&'static Config>,
    req: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let cookie = req.cookie("appCookie");

    // Get search settings using the user's cookie or from the server's config
    let preferences: server_models::Cookie<'_> = cookie
        .as_ref()
        .and_then(|cookie_value| serde_json::from_str(cookie_value.value()).ok())
        .unwrap_or_else(|| {
            server_models::Cookie::build(
                &config.style,
                config
                    .upstream_search_engines
                    .iter()
                    .filter_map(|(engine, enabled)| {
                        enabled.then_some(Cow::Borrowed(engine.as_str()))
                    })
                    .collect(),
                u8::default(),
            )
        });

    Ok(HttpResponse::Ok().json(preferences))
}

fn compute_legacy_hmac(key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    type HmacMd5 = Hmac<Md5>;

    //SINK
    let mut mac = match HmacMd5::new_from_slice(key) {
        Ok(m) => m,
        Err(_) => return Ok(()), 
    };

    mac.update(b"example payload");
    let _ = mac.finalize();

    Ok(())
}