/* eslint-disable no-new */
import React, { SFC, useRef, useEffect } from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors, base } from '../../utils';

interface Props {
  data: object;
  xLabel?: string;
  yLabel?: string;
  title?: string;
  config?: object;
}

export const XY: SFC<Props> = ({ title, data, config, xLabel, yLabel }) => {
  const ref = useRef();
  useEffect(() => {
    if (ref.current) {
      new xkcd.XY(ref.current, {
        title,
        xLabel,
        yLabel,
        data,
        options: Object.assign(config || {}, {
          legendPosition: xkcd.config.positionType.upRight,
          backgroundColor: base('07'),
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

XY.defaultProps = {
  config: {},
};
