import { Link } from 'gatsby';
import React, { SFC } from 'react';
import { container, clearfix, caret, right, path } from './styles/layout.module.scss';

interface Props {
  siteTitle?: string;
}

export const Header: SFC<Props> = ({ siteTitle }) => (
  <header>
    <div className={[container, clearfix].join(' ')}>
      <Link to="/" className={path}>
        {siteTitle}
      </Link>
      <span className={caret}>#_</span>
      <div className={right}>
        <Link to="/stock/" className={path}>
          Current Stock
        </Link>
        |
        <Link to="/history/" className={path}>
          History
        </Link>
        |
        <Link to="/graphs/" className={path}>
          Graphs
        </Link>
      </div>
    </div>
  </header>
);

Header.defaultProps = {
  siteTitle: ``,
};
