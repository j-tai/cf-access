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

use thiserror::Error;

/// An error that can occur when validating a JWT or constructing a
/// [`Validator`](super::Validator).
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Error originated from [`jwtk`].
    #[error("{0}")]
    Jwtk(#[from] jwtk::Error),
    /// Invalid audience.
    #[error("token has invalid audience")]
    InvalidAud,
    /// Missing `sub` claim in an identity token.
    #[error("token is missing 'sub' claim")]
    MissingSub,
    /// Error from [`uuid`] crate.
    #[error("UUID parse error: {0}")]
    Uuid(#[from] uuid::Error),
    /// Missing environment variable.
    #[cfg(feature = "env")]
    #[error("missing environment variable '{0}'")]
    MissingEnv(&'static str),
}
