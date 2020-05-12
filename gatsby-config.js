/* eslint-disable @typescript-eslint/camelcase, @typescript-eslint/no-var-requires, @typescript-eslint/explicit-function-return-type */
const { name, description, author, menu, project } = require('./package.json');

require('dotenv').config();

const buildCredentials = ({
  SPREADSHEET_ID,
  PROJECT_ID,
  PRIVATE_KEY,
  PRIVATE_KEY_ID,
  CLIENT_EMAIL,
  CLIENT_ID,
}) => ({
  spreadsheetId: SPREADSHEET_ID,
  credentials: {
    type: 'service_account',
    project_id: PROJECT_ID || project.id,
    private_key_id: PRIVATE_KEY_ID || project.keyId,
    private_key: (PRIVATE_KEY || project.key).replace(/(\\r)|(\\n)/g, '\n'),
    client_email: CLIENT_EMAIL || project.email,
    client_id: CLIENT_ID,
    auth_uri: 'https://accounts.google.com/o/oauth2/auth',
    token_uri: 'https://oauth2.googleapis.com/token',
    auth_provider_x509_cert_url: 'https://www.googleapis.com/oauth2/v1/certs',
    client_x509_cert_url: `https://www.googleapis.com/robot/v1/metadata/x509/${encodeURIComponent(
      CLIENT_EMAIL || project.email,
    )}`,
  },
});

module.exports = {
  siteMetadata: {
    title: name,
    description,
    author,
    menu,
  },
  plugins: [
    'gatsby-plugin-typescript',
    'gatsby-plugin-preact',
    'gatsby-plugin-sass',
    'gatsby-plugin-react-helmet',
    {
      resolve: 'gatsby-plugin-eslint',
      options: {
        test: /\.ts$|\.tsx$/,
        stages: ['develop', 'build-javascript'],
        options: {
          emitWarning: true,
          failOnError: true,
        },
      },
    },
    {
      resolve: 'gatsby-plugin-manifest',
      options: {
        name,
        short_name: name,
        start_url: '/',
        background_color: '#f2f0ec',
        theme_color: '#2d2d2d',
        icon: 'src/images/icon.png',
        display: 'minimal-ui',
      },
    },
    {
      resolve: 'gatsby-source-google-spreadsheets',
      options: buildCredentials(process.env),
    },
    'gatsby-plugin-offline',
    {
      resolve: `@gatsby-contrib/gatsby-plugin-elasticlunr-search`,
      options: {
        fields: [`name`, `brewery`, 'year', 'abv', 'style', 'collaborator'],
        resolvers: {
          googleInventorySheet: {
            name: node => node.name,
            year: ({ year }) => year,
            brewery: ({ brewery }) => brewery,
            abv: ({ abv }) => abv,
            style: ({ style }) => style,
            collaborators: ({ collaborators }) => collaborators,
          },
        },
      },
    },
  ],
};
