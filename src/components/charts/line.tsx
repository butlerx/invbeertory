import React, { FunctionComponent, useRef, useState, useEffect } from 'react';
import xkcd from 'chart.xkcd';
import { colours as dataColors } from '../../utils';

interface Props {
  data: object;
  xLabel?: string;
  yLabel?: string;
  title?: string;
  config?: object;
}

export const Line: FunctionComponent<Props> = ({ title, data, config, xLabel, yLabel }) => {
  const ref = useRef();
  useEffect(() => {
    if (ref.current) {
      const myChart = new xkcd.Line(ref.current, {
        title,
        xLabel,
        yLabel,
        data,
        options: Object.assign(config || {}, {
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

Line.defaultProps = {
  config: {},
};