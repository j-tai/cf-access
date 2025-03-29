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
