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

/**
 * The user information found in the JWT common between identity-based and
 * service-token authorization token.
 */
interface CommonToken {
    /**
     * The type of Access token (`app` for application token or `org` for global
     * session token).
     */
    type: 'app' | 'org';
    /** Application audience (AUD) tag of the Access application. */
    aud: string[];
    /** The expiration timestamp for the token (Unix time). */
    exp: number;
    /** The Cloudflare Access domain URL for the application. */
    iss: string;
    /** The issuance timestamp for the token (Unix time). */
    iat: number;
    /** Custom data set by the authorization settings. */
    custom?: unknown;

    /**
     * The email address of the authenticated user, verified by the identity
     * provider.
     */
    email?: string;
    /** The not-before timestamp for the token (Unix time). */
    nbf?: number;
    /** A cache key used to get the user's identity. */
    identity_nonce?: string;
    /**
     * The ID of the user. This contains an empty string when authentication was
     * through a service token.
     *
     * This value is unique to an email address per account.
     * The user would get a different sub if they are removed and re-added to
     * your Zero Trust organization, or if they log into a different
     * organization.
     */
    sub?: string;
    /** The country where the user authenticated from. */
    country?: string;

    /** The Client ID of the service token (`CF-Access-Client-Id`). */
    common_name?: string;
}

/**
 * The user information found in the JWT, when the user authenticated with an
 * identity provider.
 * @see https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/application-token/#identity-based-authentication
 */
export interface IdentityToken extends CommonToken {
    email: string;
    nbf: number;
    identity_nonce: string;
    sub: string;
    country: string;
    common_name?: never;
}

/**
 * The user information found in the JWT, when the client authenticated with a
 * service token.
 * @see https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/application-token/#service-token-authentication
 */
export interface ServiceToken extends CommonToken {
    email?: never;
    nbf?: never;
    identity_nonce?: never;
    sub?: never;
    country?: never;
    common_name: string;
}

/**
 * The client information found in the JWT.
 * @see https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/application-token/#payload
 */
export type Token = IdentityToken | ServiceToken;
