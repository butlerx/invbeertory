import React, { SFC } from 'react';

interface Props {
  title?: string;
}

export const Card: SFC<Props> = ({ title, children }) => (
  <div
    style={{
      border: '1px solid #eaeaea',
      boxShadow: '2px 2px 5px grey',
      padding: '10px',
      marginBottom: '10px',
      borderRadius: '10px',
    }}
  >
    {title !== undefined ? (
      <>
        <h1 style={{ paddingLeft: '1em', paddingTop: '.5em' }}>{title}</h1>
        <hr />
      </>
    ) : (
      ''
    )}
    {children}
  </div>
);
