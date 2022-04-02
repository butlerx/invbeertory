import React from 'react';
import _ from 'lodash';
import { single, headline, body, metadata, key, val } from './styles/card.module.scss';

interface Props {
  title?: string;
  meta?: { [key: string]: string | number };
}

const titleCase = (word: string): string => word.charAt(0).toUpperCase() + word.slice(1);

export const Card: React.FC<Props> = ({ title, meta, children }) => (
  <div className={single}>
    {!_.isEmpty(meta) ? (
      <div className={metadata}>
        {Object.entries(meta).map(([k, v]) => (
          <>
            <span className={key}>{titleCase(k)}:</span>
            &nbsp;
            <span className={val}>{v}</span>
            <br />
          </>
        ))}
      </div>
    ) : (
      ''
    )}

    {title !== '' ? <h1 className={headline}>{title}</h1> : ''}

    <section className={body}>{children}</section>
  </div>
);

Card.defaultProps = {
  title: '',
  meta: {},
};
