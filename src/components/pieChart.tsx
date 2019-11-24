import React, { FunctionComponent, useRef, useState, useEffect } from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors } from '../utils';

interface Props {
  data: { [string]: number };
  title: string;
  config?: { [string]: any };
}

export const Pie: FunctionComponent<Props> = ({ title, data, config }) => {
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

  return <svg ref={ref} />;
};
