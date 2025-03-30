/*
 * MIT License
 *
 * Copyright (c) 2025 Jasmine Tai
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#![doc = include_str!("../README.md")]

mod error;
mod token;

use std::time::Duration;

use jwtk::jwk::RemoteJwksVerifier;
use jwtk::Claims;
use reqwest::Client;

pub use error::Error;
pub use jwtk;
pub use reqwest;
pub use token::Token;

/// A validator for Cloudflare Access JWTs.
pub struct Validator {
    inner: RemoteJwksVerifier,
    audience: String,
}

impl Validator {
    /// Creates a new [`Validator`] with the given Cloudflare Access team name
    /// and application AUD tag.
    pub fn new(team_name: &str, audience: impl Into<String>) -> Self {
        Validator::with_client(Client::default(), team_name, audience)
    }

    /// Creates a new [`Validator`] from the current process's environment
    /// variables.
    #[cfg(feature = "env")]
    pub fn from_env() -> Result<Self, Error> {
        fn var(name: &'static str) -> Result<String, Error> {
            std::env::var(name).map_err(|_| Error::MissingEnv(name))
        }

        let team_name = var("CF_ACCESS_TEAM")?;
        let audience = var("CF_ACCESS_AUD")?;
        Ok(Validator::new(&team_name, audience))
    }

    /// Creates a new [`Validator`] that uses a specific [`reqwest::Client`].
    pub fn with_client(client: Client, team_name: &str, audience: impl Into<String>) -> Self {
        let issuer = format!("https://{team_name}.cloudflareaccess.com");
        let url = format!("{issuer}/cdn-cgi/access/certs");
        Validator {
            inner: RemoteJwksVerifier::new(url, Some(client), CACHE_DURATION),
            audience: audience.into(),
        }
    }

    /// Validates the JWT.
    pub async fn validate(&self, jwt: &str) -> Result<Claims<Token>, Error> {
        let mut token = self.inner.verify(jwt).await?;
        if !token.claims().aud.iter().any(|aud| **aud == self.audience) {
            return Err(Error::InvalidAud);
        }

        Ok(std::mem::take(token.claims_mut()))
    }
}

const CACHE_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 3); // 3 days
