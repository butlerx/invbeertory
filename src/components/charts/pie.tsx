/* eslint-disable no-new */
import React from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors, base } from '../../utils';

interface Props {
  data: { [key: string]: number };
  title?: string;
  config?: Record<string, unknown>;
}

export function Pie({ title, data, config }: Props): React.ReactElement {
  const ref = React.useRef();
  React.useEffect(() => {
    if (ref.current) {
      new xkcd.Pie(ref.current, {
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
          backgroundColor: base('07'),
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

Pie.defaultProps = {
  title: '',
  config: {},
};
