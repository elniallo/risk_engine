# risk_engine
## To Use
Requires a kafka producer at localhost:9082 - topic "test"
```json
{"user_id":100,"bought_token":"ETH","bought_quantity":87.35,"sold_token":"BTC","sold_quantity":6.9}
```
Rest API at port 7878 -> localhost:7878/api/v1/withdrawBalance
```json
{
	"user_id": 100,
	"amount": 7,
	"order_type": "BTC"
}
```

## Running Instructions
- Make sure you have rust installed, see [here](https://www.rust-lang.org/tools/install)
- git clone https://github.com/elniallo/risk_engine.git
- cargo run
