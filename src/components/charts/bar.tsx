/* eslint-disable no-new */
import React from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors, base } from '../../utils';

interface Props {
  data: { [key: string]: number };
  xLabel?: string;
  yLabel?: string;
  title?: string;
  config?: Record<string, unknown>;
}

export function Bar({ title, data, config, xLabel, yLabel }: Props): React.ReactElement {
  const ref = React.useRef();
  React.useEffect(() => {
    if (ref.current) {
      new xkcd.Bar(ref.current, {
        title,
        xLabel,
        yLabel,
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
}

Bar.defaultProps = {
  config: {},
  title: '',
  xLabel: '',
  yLabel: '',
};
