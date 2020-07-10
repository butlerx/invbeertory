import React, { SFC } from 'react';
import { Link } from 'gatsby';

import { Layout, SEO, Deck, Card } from '../components';
import { pageNotFound } from '../components/styles/404.module.scss';
import { path } from '../components/styles/layout.module.scss';
import { base05, base08, base09, base0a } from '../components/styles/colours.module.scss';

const error: SFC<{}> = () => (
  <Layout>
    <SEO title="404: Not found" />
    <div className={pageNotFound}>
      <h1>
        <span className={base05}>[</span>
        <span className={base08}>4</span>
        <span className={base09}>0</span>
        <span className={base0a}>4</span>
        <span className={base05}>]</span>
        <span className={base05}># _</span>
      </h1>
      <Deck>
        <Card>
          <p>You just hit a route that doesn&#39;t exist... the sadness.</p>
          <p>
            Please head back{' '}
            <Link className={path} to="/">
              home
            </Link>
            .
          </p>
        </Card>
      </Deck>
    </div>
  </Layout>
);

export default error;
