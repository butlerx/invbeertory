import React, { SFC } from 'react';

import { single, headline, body } from './styles/card.module.scss';

interface Props {
  title?: string;
}

export const Card: SFC<Props> = ({ title, children }) => (
  <div className={single}>
    {title !== undefined ? (
      <>
        <h1 className={headline}>{title}</h1>
      </>
    ) : (
      ''
    )}

    <section className={body}>{children}</section>
  </div>
);
