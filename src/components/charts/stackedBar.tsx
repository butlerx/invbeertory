/* eslint-disable no-new */
import React, { SFC, useRef, useEffect } from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors, base } from '../../utils';

interface Props {
  data: { [key: string]: { [key: string]: number } };
  xLabel?: string;
  yLabel?: string;
  title?: string;
  config?: Record<string, unknown>;
}

export const StackedBar: SFC<Props> = ({ title, data, config, xLabel, yLabel }) => {
  const ref = useRef();
  useEffect(() => {
    if (ref.current) {
      new xkcd.StackedBar(ref.current, {
        title,
        xLabel,
        yLabel,
        data,
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

StackedBar.defaultProps = {
  title: '',
  config: {},
};
