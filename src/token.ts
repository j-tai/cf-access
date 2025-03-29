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
