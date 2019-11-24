import React, { SFC, useRef, useState, useEffect } from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors } from '../../utils';

interface Props {
  data: { [string]: number };
  title?: string;
  config?: object;
}

export const Pie: SFC<Props> = ({ title, data, config }) => {
  const ref = useRef();
  useEffect(() => {
    if (ref.current) {
      const myChart = new xkcd.Pie(ref.current, {
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
          innerRadius: 0.4,
          legendPosition: xkcd.config.positionType.upRight,
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

Pie.defaultProps = {
  title: '',
  config: {},
};
