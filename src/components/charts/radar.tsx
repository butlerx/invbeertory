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

export function Radar({ title, data, config, xLabel, yLabel }: Props): React.ReactElement {
  const ref = React.useRef();
  React.useEffect(() => {
    if (ref.current) {
      new xkcd.Radar(ref.current, {
        title,
        xLabel,
        yLabel,
        data,
        options: Object.assign(config || {}, {
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

Radar.defaultProps = {
  title: '',
  config: {},
  xLabel: '',
  yLabel: '',
};
