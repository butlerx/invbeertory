import { Link } from 'gatsby';
import React, { SFC } from 'react';

interface Props {
  siteTitle?: string;
}

export const Header: SFC<Props> = ({ siteTitle }) => (
  <header
    style={{
      background: `rebeccapurple`,
      marginBottom: `1.45rem`,
    }}
  >
    <div
      style={{
        margin: `0 auto`,
        maxWidth: 960,
        padding: `1.45rem 1.0875rem`,
        display: 'flex',
        flexGrow: 1,
      }}
    >
      <h1 style={{ margin: 0 }}>
        <Link
          to="/"
          style={{
            color: `white`,
            textDecoration: `none`,
          }}
        >
          {siteTitle}
        </Link>
      </h1>
      <span
        style={{
          flexGrow: 1,
        }}
      />
      <div style={{ margin: 0, paddingTop: '1em' }}>
        <Link
          to="/graphs/"
          style={{
            color: `white`,
            textDecoration: `none`,
          }}
        >
          Graphs
        </Link>
      </div>
    </div>
  </header>
);

Header.defaultProps = {
  siteTitle: ``,
};
