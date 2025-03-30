# cf-access

Simple Cloudflare Access JWT validator.

## Usage

You will need:

* Your team name
* Your application's [AUD tag](https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/validating-json/#get-your-aud-tag)

```no_run
# #[tokio::main]
# async fn main() {
use cf_access::Validator;

let validator = Validator::new("team_name", "aud_tag");

# let mut headers = std::collections::HashMap::new();
# headers.insert("cf-access-jwt-assertion", "...");
if let Some(jwt) = headers.get("cf-access-jwt-assertion") {
    if let Ok(claims) = validator.validate(jwt).await {
        println!("{claims:?}");
    }
}
# }
```

## License

MIT
