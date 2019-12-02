import React, { SFC } from 'react';

import { deck } from './styles/deck.module.scss';

export const Deck: SFC<{}> = ({ children }) => <div className={deck}>{children}</div>;
