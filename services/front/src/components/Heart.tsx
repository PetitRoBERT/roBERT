import React from 'react';
import { keyframes } from '@emotion/core';
import propTypes from 'prop-types';
import styled from '@emotion/styled';

const pulse = keyframes`
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.3);
  }
  100% {
    transform: scale(1);
  }
`;

const SVGBase = styled.svg`
  fill: red;
  position: relative;
  top: 5px;
  width: 50px;
  animation: ${pulse} 0.8s ease infinite;
`;

type HeartProps = {
  height?: number;
};

const Heart: React.FunctionComponent<HeartProps> = ({ height }) => (
  <SVGBase viewBox="0 0 32 29.6" height={height}>
    <path
      d="M23.6,0c-3.4,0-6.3,2.7-7.6,5.6C14.7,2.7,11.8,0,
         8.4,0C3.8,0,0,3.8,0,8.4c0,9.4,9.5,11.9,16,21.2
         c6.1-9.3,16-12.1,16-21.2C32,3.8,28.2,0,23.6,0z"
    />
  </SVGBase>
);

Heart.defaultProps = {
  height: 100,
};

Heart.propTypes = {
  height: propTypes.number,
};

export default Heart;
