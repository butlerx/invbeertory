import React, { SFC } from 'react';

interface Props {
  title: string;
  date: Date;
  content: string;
}

export const Card: SFC<Props> = ({ title, content, date }) => (
  <div
    style={{
      border: '1px solid #eaeaea',
      boxShadow: '2px 2px 5px grey',
      padding: '10px',
      marginBottom: '10px',
      borderRadius: '10px',
    }}
  >
    <h1>{title}</h1>
    <p style={{ color: 'grey' }}>{date}</p>
    <p>{content}</p>
  </div>
);
