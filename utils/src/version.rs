use crate::imports::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl AsRef<Version> for Version {
    fn as_ref(&self) -> &Version {
        self
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split('.');
        let major = parts
            .next()
            .ok_or_else(|| Error::custom("Invalid version"))?
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()?;
        let minor = parts
            .next()
            .ok_or_else(|| Error::custom("Invalid version"))?
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()?;
        let patch = parts
            .next()
            .ok_or_else(|| Error::custom("Invalid version"))?
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()?;
        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn is_greater_than<V>(&self, other: V) -> bool
    where
        V: AsRef<Version>,
    {
        use std::cmp::Ordering;

        let other = other.as_ref();

        matches!(
            (
                self.major.cmp(&other.major),
                self.minor.cmp(&other.minor),
                self.patch.cmp(&other.patch),
            ),
            (Ordering::Greater, _, _)
                | (Ordering::Equal, Ordering::Greater, _)
                | (Ordering::Equal, Ordering::Equal, Ordering::Greater)
        )
    }
}

#[derive(Debug, Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    crate_: Crate,
}

#[derive(Debug, Deserialize)]
struct Crate {
    max_version: String,
}

pub async fn latest_crate_version<S: Display, U: Display>(
    crate_name: S,
    user_agent: U,
) -> Result<Version> {
    let url = format!("https://crates.io/api/v1/crates/{crate_name}");
    let response = http::Request::new(url)
        .with_user_agent(user_agent.to_string())
        .get_json::<CrateResponse>()
        .await?;
    response.crate_.max_version.parse()
}

#[cfg(not(target_arch = "wasm32"))]
pub mod blocking {
    use super::*;
    use reqwest::blocking::Client;
    use reqwest::header::*;

    pub fn latest_crate_version<S: Display, U: Display>(
        crate_name: S,
        user_agent: U,
    ) -> Result<Version> {
        let url = format!("https://crates.io/api/v1/crates/{crate_name}");
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(user_agent.to_string().as_str())?,
        );
        let response = Client::builder()
            .default_headers(headers)
            .build()?
            .get(url)
            .send()?
            .json::<CrateResponse>()?;
        response.crate_.max_version.parse()
    }
}
