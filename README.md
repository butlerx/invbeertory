# Welcome to invbeertory 👋

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000)
[![Documentation](https://img.shields.io/badge/documentation-yes-brightgreen.svg)](https://github.com/butlerx/invbeertory#readme)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/butlerx/invbeertory/graphs/commit-activity)
[![License: MIT](https://img.shields.io/github/license/butlerx/invbeertory)](https://github.com/butlerx/invbeertory/blob/master/LICENSE)
[![Twitter: cianbutlerx](https://img.shields.io/twitter/follow/cianbutlerx.svg?style=social)](https://twitter.com/cianbutlerx)

> Simple beer inventory displaty

### ✨ [Demo](beer.notthe.cloud)

## Install

```sh
yarn
```

## Usage

Before building you will need to add credentials to the `.env` file.

```sh
SPREADSHEET_ID=<YOUR_SPREADHEET_ID>
CLIENT_EMAIL=<SERVICE_ACCOUNT_EMAIL>
PRIVATE_KEY=<YOUR_PRIVATE_KEY>
```

You can get the `SPREADSHEET_ID` from the URL of your sheet, `https://docs.google.com/spreadsheets/d/<SPREADSHEET_ID>/edit#gid=12345`.
You will have to create a service account which will provide you with the service account email id and a private key. To know more about creating service account you can visit [here](https://support.google.com/a/answer/7378726?hl=en) and read [this](https://www.twilio.com/blog/2017/03/google-spreadsheets-and-javascriptnode-js.html). After creating a service account share your sheet with edit permission with the given email id.

```sh
yarn build
```

## Author

👤 **Cian Butler <butlerx@notthe.cloud>**

- Website: cianbutler.ie
- Twitter: [@cianbutlerx](https://twitter.com/cianbutlerx)
- Github: [@butlerx](https://github.com/butlerx)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/butlerx/invbeertory/issues).

## Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2019 [Cian Butler <butlerx@notthe.cloud>](https://github.com/butlerx).

This project is [MIT](https://github.com/butlerx/invbeertory/blob/master/LICENSE) licensed.

---

_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
