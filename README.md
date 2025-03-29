# @jtai/cf-access

Simple Cloudflare Access JWT validator.

## Usage

You will need:

* Your team name
* Your application's [AUD tag](https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/validating-json/#get-your-aud-tag)

```ts
import { createValidator } from '@jtai/cf-access';

const validate = createValidator('teamName', 'audTag');
const jwt = req.headers['cf-access-jwt-assertion'];
const claims = validate.orThrow(jwt);
console.log(claims);
```
