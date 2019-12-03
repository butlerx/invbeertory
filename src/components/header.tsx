import { Link } from 'gatsby';
import React, { SFC } from 'react';
import { container, clearfix, caret, right, path } from './styles/layout.module.scss';

interface Props {
  menu: { name: string; link: string }[];
  siteTitle?: string;
}

export const Header: SFC<Props> = ({ siteTitle, menu }) => (
  <header>
    <div className={[container, clearfix].join(' ')}>
      <Link to="/" className={path}>
        {siteTitle}
      </Link>
      <span className={caret}> #_</span>
      <div className={right}>
        {menu.map(({ link, name }, i) => (
          <>
            {i != 0 ? ' | ' : ''}
            <Link to={link} className={path}>
              {name}
            </Link>
          </>
        ))}
      </div>
    </div>
  </header>
);

Header.defaultProps = {
  siteTitle: ``,
};
