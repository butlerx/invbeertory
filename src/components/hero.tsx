import React, { SFC } from 'react';
import { base } from '../utils';
import * as colours from './styles/colours.module.scss';
import { siteTitle } from './styles/layout.module.scss';

interface Props {
  title: string;
  message: string;
}

export const Hero: SFC<Props> = ({ title, message }) => (
  <>
    <h1 className={siteTitle}>
      <span className={colours.base05}>[</span>
      {title.split('').map((letter, i) => (
        <span
          className={
            colours[
              `base${['03', '04', '05', '06', '07', '08', '09', '0a', '0b', '0c', '0d', 'e0'][i]}`
            ]
          }
        >
          {letter}
        </span>
      ))}
      <span className={colours.base05}>]</span>
      <span className={colours.base05}>#_</span>
    </h1>

    <div className={colours.heroLogo}></div>

    <div style={{ textAlign: 'center' }}>{message}</div>
  </>
);
