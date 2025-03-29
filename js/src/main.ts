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

import express from 'express';
import { createValidatorFromEnv } from './lib.js';
import { argv, env, exit } from 'process';

const validate = createValidatorFromEnv(env);

if (argv.length > 2) {
    for (const arg of argv.slice(2)) {
        console.log(await validate.orThrow(arg));
    }
    exit(0);
}

const app = express();
app.get('/', async (req, res) => {
    const jwt = req.headers['cf-access-jwt-assertion'];
    if (typeof jwt !== 'string') {
        res.json({ error: 'JWT not found' });
        return;
    }

    const result = await validate.orNull(jwt);
    if (result) {
        res.json({ result, jwt });
    } else {
        res.json({ error: 'JWT is invalid', jwt });
    }
});

const port = 3000;
app.listen(port, () => {
    console.log(`Now listening on port ${port}...`);
});
