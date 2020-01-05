use crate::http::{base_headers, get_http_client};
use crate::media_container::ServerMediaContainer;
use crate::server::Server;

impl Server {
    /// Establish a connection with the server server by provided url.
    ///
    /// This call will fail if anonymous access is denied.
    pub async fn connect(url: &str) -> crate::Result<Self> {
        let response = get_http_client()?
            .get(url)
            .headers(base_headers())
            .send()
            .await?;
        if response.status() == reqwest::StatusCode::OK {
            let mc: ServerMediaContainer =
                quick_xml::de::from_str(response.text().await?.as_str())?;
            Ok(Server {
                info: mc,
                url: String::from(url),
                auth_token: String::from(""),
            })
        } else {
            eprintln!("{:?}", response.text().await?);
            Err(crate::error::PlexApiError {})
        }
    }

    /// Establish a connection with the server server by provided url and [`authentication token`].
    ///
    /// [`authentication token`]: struct.MyPlexAccount.html#method.get_auth_token
    pub async fn login(url: &str, auth_token: &str) -> crate::Result<Self> {
        let response = get_http_client()?
            .get(url)
            .headers(base_headers())
            .header("X-Plex-Token", auth_token)
            .send()
            .await?;
        if response.status() == reqwest::StatusCode::OK {
            let mc: ServerMediaContainer =
                quick_xml::de::from_str(response.text().await?.as_str())?;
            Ok(Server {
                info: mc,
                url: String::from(url),
                auth_token: String::from(auth_token),
            })
        } else {
            eprintln!("{:?}", response.text().await?);
            Err(crate::error::PlexApiError {})
        }
    }
}
