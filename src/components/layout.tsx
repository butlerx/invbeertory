import React, { SFC, ReactElement } from 'react';
import { StaticQuery, graphql } from 'gatsby';

import { getMonth } from '../utils';
import { Header } from './header';
import { container } from './styles/layout.module.scss';
import './styles/main.scss';

export const Layout: SFC<Record<string, never>> = ({ children }) => (
  <StaticQuery
    query={graphql`
      query SiteTitleQuery {
        site {
          siteMetadata {
            title
            author
            menu {
              name
              link
            }
          }
        }
      }
    `}
    render={({
      site: {
        siteMetadata: { title, author, menu },
      },
    }): ReactElement => (
      <>
        <Header siteTitle={title} menu={menu} />
        <div className={container}>
          <main>{children}</main>
        </div>
        <footer>
          <div className={container}>
            <span>
              &copy; {getMonth(new Date().getMonth())}, {new Date().getFullYear()} {author} -{' '}
              <a rel="license" href="http://creativecommons.org/licenses/by/4.0/">
                {' '}
                CC BY 4.0
              </a>
            </span>
          </div>
        </footer>
      </>
    )}
  />
);
