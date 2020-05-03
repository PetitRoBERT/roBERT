import PropTypes from 'prop-types';
import React from 'react';

interface Props {
  text?: string;
}

const TypedComponent: React.FC<Props> = ({ text }) => {
  return <p>{text}</p>;
};

TypedComponent.defaultProps = {
  text: ' default value',
};

TypedComponent.propTypes = {
  text: PropTypes.string,
};

export default TypedComponent;
