import React from 'react';

import { deck } from './styles/deck.module.scss';

export const Deck: React.FC<Record<string, never>> = ({ children }) => (
  <div className={deck}>{children}</div>
);
