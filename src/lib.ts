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
