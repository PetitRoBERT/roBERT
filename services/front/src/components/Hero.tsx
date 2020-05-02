import { Flex, Heading } from '@chakra-ui/core';
import React, { FunctionComponent } from 'react';
import propTypes from 'prop-types';

const Hero: FunctionComponent = props => (
  <Flex justifyContent="center" alignItems="center" height="100vh">
    <Heading fontSize="50px">{props.children}</Heading>
  </Flex>
);

Hero.propTypes = {
  children: propTypes.object.isRequired,
};

export default Hero;
