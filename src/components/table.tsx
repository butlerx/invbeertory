import React, { FunctionComponent } from 'react';

interface Props {
  headings: string[];
  data: object[];
}

export const Table: FunctionComponent<Props> = ({ data, headings }) => (
  <>
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: '3fr 1fr 1fr 0.5fr',
        fontWeight: 'bold',
      }}
    >
      {headings.map(heading => (
        <div>{heading}</div>
      ))}
    </div>
    {data.map(item => (
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: '3fr 1fr 1fr 0.5fr',
        }}
      >
        {headings.map(heading => (
          <div>{item['heading']}</div>
        ))}
      </div>
    ))}
  </>
);
