import React, { SFC } from 'react';

import { single, headline, body, metadata, key, val } from './styles/card.module.scss';

interface Props {
  title?: string;
  meta?: { [key: string]: string };
}

const titleCase = (word: string) => word.charAt(0).toUpperCase() + word.slice(1);

export const Card: SFC<Props> = ({ title, meta, children }) => (
  <div className={single}>
    {meta !== undefined ? (
      <div className={metadata}>
        {Object.entries(meta).map(([k, v]) => (
          <>
            <span className={key}>{titleCase(k)}:</span>&nbsp;
            <span className={val}>{v}</span>
            <br />
          </>
        ))}
      </div>
    ) : (
      ''
    )}

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
