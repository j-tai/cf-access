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

/// The user information found in the JWT.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Token {
    /// The user authenticated with an identity provider.
    Identity {
        /// The email address of the authenticated user, verified by the identity provider.
        email: String,
        /// A cache key used to get the user's identity.
        identity_nonce: String,
        /// The country where the user authenticated from.
        country: String,
    },
    /// The client authenticated with a service token.
    Service {
        /// The Client ID of the service token (`CF-Access-Client-Id`).
        common_name: String,
    },
}

impl Default for Token {
    fn default() -> Self {
        Token::Service {
            common_name: "".into(),
        }
    }
}
