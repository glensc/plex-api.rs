use crate::media_container::{Device, MediaContainer};
use crate::my_plex::{MyPlexAccount, MyPlexError, Result, MyPlexApiErrorResponse};
use reqwest::StatusCode;
use serde_xml_rs;

impl MyPlexAccount {
    pub fn get_resources(&self) -> Result<Vec<Device>> {
        let mut response = self.get("https://plex.tv/api/resources")?;
        if response.status() == StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            let resources = mc.get_devices();
            if resources.is_some() {
                Ok(resources.unwrap())
            } else {
                let ret: Vec<Device> = Vec::new();
                Ok(ret)
            }
        } else {
            let err: MyPlexApiErrorResponse = serde_xml_rs::from_str(response.text()?.as_str())?;
            Err(MyPlexError::from(err))
        }
    }
}
