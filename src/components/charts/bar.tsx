import React, { FunctionComponent, useRef, useState, useEffect } from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors } from '../../utils';

interface Props {
  data: { [string]: number };
  title?: string;
  config?: object;
}

export const Bar: FunctionComponent<Props> = ({ title, data, config }) => {
  const ref = useRef();
  useEffect(() => {
    if (ref.current) {
      const myChart = new xkcd.Bar(ref.current, {
        title,
        data: {
          labels: Object.keys(data),
          datasets: [
            {
              data: Object.values(data),
            },
          ],
        },
        options: Object.assign(config || {}, {
          yTickCount: 10,
          dataColors,
        }),
      });
    }
  });

  return (
    <div style={{ width: '100%', height: 'auto' }}>
      <svg ref={ref} />
    </div>
  );
};

Bar.defaultProps = {
  title: '',
  config: {},
};
