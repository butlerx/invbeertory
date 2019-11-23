const { name, description, author } = require('./package.json');

require('dotenv').config({
  path: `.env.${process.env.NODE_ENV}`,
});

const { client_email, private_key } = require(`./${process.eve.SERVICEACCOUNT}.json`);

module.exports = {
  siteMetadata: {
    title: package.name,
    description,
    author,
  },
  plugins: [
    'gatsby-plugin-typescript',
    'gatsby-plugin-react-helmet',
    {
      resolve: 'gatsby-source-filesystem',
      options: {
        name: 'images',
        path: `${__dirname}/src/images`,
      },
    },
    'gatsby-transformer-sharp',
    'gatsby-plugin-sharp',
    {
      resolve: 'gatsby-plugin-manifest',
      options: {
        name: 'gatsby-starter-default',
        short_name: 'starter',
        start_url: '/',
        background_color: '#663399',
        theme_color: '#663399',
        display: 'minimal-ui',
        icon: 'src/images/gatsby-icon.png', // This path is relative to the root of the site.
      },
    },
    {
      resolve: 'gatsby-source-google-spreadsheet',
      options: {
        spreadsheetId: process.env.SPREADSHEET_ID,
        credentials: {
          client_email,
          private_key,
        },
      },
    },
    'gatsby-plugin-offline',
  ],
};
