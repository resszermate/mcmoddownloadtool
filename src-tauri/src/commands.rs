use log::info;
use reqwest::Client;
use serde::Deserialize;

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
    // add other fields if needed
}

#[tauri::command]
pub async fn download_mods(file_list: FileList)  -> Result<(), String>{
    let client = Client::new();

    let mods = file_list.files_list;
    info!("Mod names : {:?}", mods);
     for single_mod in mods {
        let mod_string = single_mod.as_str().split("-").next();
        let params = [("query", mod_string)];
        info!("{:?}",mod_string);
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
            info!("Slug: {}", hit.slug);
        }
    }
    Ok(())
}
