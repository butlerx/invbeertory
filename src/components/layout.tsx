import React, { SFC } from 'react';
import { StaticQuery, graphql } from 'gatsby';

import { getMonth } from '../utils';
import { Header } from './header';
import { container } from './styles/layout.module.scss';
import './styles/main.scss';

export const Layout: SFC<{}> = ({ children }) => (
  <StaticQuery
    query={graphql`
      query SiteTitleQuery {
        site {
          siteMetadata {
            title
            author
          }
        }
      }
    `}
    render={data => (
      <>
        <Header siteTitle={data.site.siteMetadata.title} />
        <div className={container}>
          <main>{children}</main>
        </div>
        <footer>
          <div className={container}>
            <span>
              &copy; {getMonth(new Date().getMonth())}, {new Date().getFullYear()}{' '}
              {data.site.author} -{' '}
              <a rel="license" href="http://creativecommons.org/licenses/by/4.0/">
                CC BY 4.0
              </a>
            </span>
          </div>
        </footer>
      </>
    )}
  />
);
