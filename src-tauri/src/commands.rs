use std::{fs::File, io};

use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Deserialize)]
pub struct FileList {
    files_list: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ModrinthSearchResponse {
    hits: Vec<ModrinthHit>,
}

#[derive(Deserialize, Debug)]
struct ModrinthHit {
    slug: String, 
    latest_version: String,                                                 
}

#[derive(Deserialize, Debug, Serialize)]
struct ProjectVersionFile {
    url: String,
    filename: String,
    primary: bool,

}

#[derive(Deserialize, Debug,Serialize)]
struct ProjectVersionResponse {
    files: Vec<ProjectVersionFile>,
    loaders: Vec<String>,
    game_versions: Vec<String>,

}

#[tauri::command]
pub async fn download_mods(file_list: FileList)  -> Result<(), String>{
    let client = Client::new();

    let mods = file_list.files_list;
    // info!("Mod names : {:?}", mods);
     for single_mod in mods {
        let mod_string = single_mod.split(&['-', '_', ':', '@'][..]).next().unwrap_or("");
        let params = [("query", mod_string)];
        
        // info!("{:?}",mod_string);
        let resp = client
            .get("https://api.modrinth.com/v2/search")
            .form(&params)
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json::<ModrinthSearchResponse>()
            .await
            .map_err(|err| err.to_string())?;

        for hit in resp.hits {
            // info!("Slug: {}", hit.slug);
            // info!("Latest version: {}",hit.latest_version);
            let version_url = format!("https://api.modrinth.com/v2/project/{}/version",hit.slug);
            info!("{}",version_url);
            let version_params = [("loaders","fabric"),("game_versions",&hit.latest_version)];

            let resp_version = client.
            get(version_url).
            form(&version_params).
            send().
            await.
            map_err(|err| err.to_string())?.
            json::<Vec<ProjectVersionResponse>>().
            await.
            map_err(|err|err.to_string())?;

            
           if let Some(version) = resp_version.iter().find(|v| v.loaders.iter().any(|l| l == "fabric")) {
            if let Some(file) = version.files.iter().find(|f| f.primary).or_else(|| version.files.first()) {
                info!("Download URL: {}", file.url);
                let file_resp = reqwest::get(&file.url).await.expect("Request failed!");
                let file_body = file_resp.text().await.expect("Invalid body");
                let filename = format!("./{}",file.url.split("/").last().unwrap_or(""));
                let mut out = File::create(filename).expect("Failed to create file");
                io::copy(&mut file_body.as_bytes(), &mut out).expect("Failed to copy content");

            }
}

            }   

    }
    Ok(())
}
