import { createRemoteJWKSet, jwtVerify } from 'jose';
import { Token } from './token.js';

/** A validator for Cloudflare Access JWTs. */
export interface Validator {
    /** Validates the JWT, or throws an error if it is invalid. */
    orThrow(jwt: string | null | undefined): Promise<Token>;
    /** Validates the JWT, or returns null if it is invalid. */
    orNull(jwt: string | null | undefined): Promise<Token | null>;
}

/**
 * Creates a new validator.
 * @param teamName The team name, e.g., `myteam`
 * @param audience The application's AUD tag
 * @returns A validator configured with the provided options
 */
export function createValidator(teamName: string, audience: string): Validator {
    const teamDomain = `https://${teamName}.cloudflareaccess.com`;
    const jwks = createRemoteJWKSet(new URL(teamDomain + '/cdn-cgi/access/certs'));

    return {
        async orThrow(jwt) {
            if (!jwt) throw new Error('No Cloudflare Access JWT provided');
            const result = await jwtVerify(jwt, jwks, {
                issuer: teamDomain,
                audience,
            });
            return result.payload as unknown as Token;
        },

        async orNull(jwt) {
            try {
                return await this.orThrow(jwt);
            } catch {
                return null;
            }
        },
    };
}

/**
 * Creates a new validator from environment variables. If a required environment
 * variable is missing, then this throws an exception.
 * @param env The environment variables object
 * @returns A validator configured with the provided environment variables
 */
export function createValidatorFromEnv(env: Record<string, string | undefined>): Validator {
    const findVar = (varName: string) => {
        if (!env[varName]) {
            throw new Error(`Environment variable '${varName}' is missing`);
        }
        return env[varName];
    };
    return createValidator(findVar('CF_ACCESS_TEAM'), findVar('CF_ACCESS_AUD'));
}

export { IdentityToken, ServiceToken, Token } from './token.js';
