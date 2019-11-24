import React, { SFC } from 'react';
import PropTypes from 'prop-types';
import { StaticQuery, graphql } from 'gatsby';
import { getMonth } from '../utils';

import { Header } from './header';
import './layout.css';

interface Props {}

export const Layout: SFC<Props> = ({ children }) => (
  <StaticQuery
    query={graphql`
      query SiteTitleQuery {
        site {
          siteMetadata {
            title
          }
        }
      }
    `}
    render={data => (
      <>
        <Header siteTitle={data.site.siteMetadata.title} />
        <div
          style={{
            margin: `0 auto`,
            minWidth: 960,
            padding: `0px 1.0875rem 1.45rem`,
            paddingTop: 0,
          }}
        >
          <main>{children}</main>
          <footer
            style={{
              alignItems: 'center',
              display: 'flex',
              flexDirection: 'row',
              justifyContent: 'center',
              padding: '1em',
            }}
          >
            Built on {getMonth(new Date().getMonth())}, {new Date().getFullYear()}
          </footer>
        </div>
      </>
    )}
  />
);
