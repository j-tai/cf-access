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

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The user information found in the JWT.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Claims {
    /// The user authenticated with an identity provider.
    Identity(IdentityClaims),
    /// The client authenticated with a service token.
    Service(ServiceClaims),
}

/// The information in the JWT when authenticating with an identity provider.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityClaims {
    /// The ID of the user.
    ///
    /// This value is unique to an email address per account. The user would get
    /// a different sub if they are removed and re-added to your Zero Trust
    /// organization, or if they log into a different organization.
    pub sub: Uuid,
    /// The email address of the authenticated user, verified by the identity
    /// provider.
    pub email: String,
    /// The type of Access token.
    #[serde(rename = "type")]
    pub ty: TokenType,
    /// A cache key used to get the user's identity.
    pub identity_nonce: String,
    /// The country where the user authenticated from.
    pub country: String,
}

/// The information in the JWT when authenticating with a service token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceClaims {
    /// The type of Access token.
    #[serde(rename = "type")]
    pub ty: TokenType,
    /// The Client ID of the service token (`CF-Access-Client-Id`).
    pub common_name: String,
}

/// The type of token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    /// An application token.
    #[serde(rename = "app")]
    Application,
    /// A global session token. (Serialized as `"org"`.)
    #[serde(rename = "org")]
    Global,
}

impl From<IdentityClaims> for Claims {
    fn from(value: IdentityClaims) -> Self {
        Claims::Identity(value)
    }
}

impl From<ServiceClaims> for Claims {
    fn from(value: ServiceClaims) -> Self {
        Claims::Service(value)
    }
}

/// The fields of [`Claims`] that don't appear in [`jwtk::Claims`].
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ClaimsExtra {
    Identity {
        email: String,
        identity_nonce: String,
        country: String,
        #[serde(rename = "type")]
        ty: TokenType,
    },
    Service(ServiceClaims),
}

impl Default for ClaimsExtra {
    fn default() -> Self {
        ClaimsExtra::Service(ServiceClaims {
            common_name: "".into(),
            ty: TokenType::Application,
        })
    }
}
