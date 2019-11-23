import { Link } from 'gatsby';
import React, { SFC } from 'react';
import { Layout, SEO } from '../components';

interface ErrorProps {}

const error: SFC<ErrorProps> = props => {
  <Layout>
    <SEO title="404: Not found" />
    <h1>404 Not Found</h1>
    <p>You just hit a route that doesn&#39;t exist... the sadness.</p>
    <p>
      Please head back <Link to="/">home</Link>.
    </p>
  </Layout>;
};

export default error;
