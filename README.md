# Telegram message decoder bot
A Telegram bot for decoding replied messages.

## Building and running the Application
### Docker
Build and start all services:

```bash
docker compose up --build
```
---
## Configuration
### Common Configuration
- **APP_MODEL_TYPE**: Usage model type. Available values:
  - **yandex**
- **APP_TELEGRAM_TOKEN**: Your Telegram bot token.

### Yandex Model Configuration
- **APP_YANDEX_API_TOKEN**: Yandex API key token for [Yandex Cloud API](https://yandex.cloud/en/docs/iam/concepts/authorization/api-key)
- **APP_YANDEX_CATALOG_ID**: Yandex Cloud [catalog](https://yandex.cloud/ru/docs/resource-manager/concepts/resources-hierarchy#folder) id

## How to work with bot
To use the bot, quote a message and mention the bot using `@bot-username`.
