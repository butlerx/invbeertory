const { name, description, author, menu, project } = require('./package.json');

require('dotenv').config();

const buildCredentials = ({
  SPREADSHEET_ID,
  PROJECT_ID,
  PRIVATE_KEY,
  PRIVATE_KEY_ID,
  CLIENT_EMAIL,
}) => ({
  spreadsheetId: SPREADSHEET_ID,
  worksheetTitle: 'beer_inv',
  credentials: {
    type: 'service_account',
    project_id: PROJECT_ID || project.id,
    private_key_id: PRIVATE_KEY_ID || project.keyId,
    private_key: (PRIVATE_KEY || project.key).replace(/(\\r)|(\\n)/g, '\n'),
    client_email: CLIENT_EMAIL || project.email,
    client_id: '',
    auth_uri: 'https://accounts.google.com/o/oauth2/auth',
    token_uri: 'https://oauth2.googleapis.com/token',
    auth_provider_x509_cert_url: 'https://www.googleapis.com/oauth2/v1/certs',
    client_x509_cert_url: `https://www.googleapis.com/robot/v1/metadata/x509/${PROJECT_ID ||
      project.id}%40appspot.gserviceaccount.com`,
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
    'gatsby-plugin-sass',
    'gatsby-plugin-react-helmet',
    {
      resolve: 'gatsby-plugin-manifest',
      options: {
        name,
        short_name: name,
        start_url: '/',
        background_color: '#2d2d2d',
        theme_color: '#f2f0ec',
        display: 'minimal-ui',
      },
    },
    {
      resolve: 'gatsby-source-google-sheets',
      options: buildCredentials(process.env),
    },
    'gatsby-plugin-offline',
  ],
};
