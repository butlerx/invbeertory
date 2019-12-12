/* eslint-disable no-new */
import React, { SFC, useRef, useEffect } from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors, base } from '../../utils';

interface Props {
  data: { [key: string]: number };
  title?: string;
  config?: object;
}

export const Bar: SFC<Props> = ({ title, data, config }) => {
  const ref = useRef();
  useEffect(() => {
    if (ref.current) {
      new xkcd.Bar(ref.current, {
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
          backgroundColor: base('07'),
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
