import { Flex } from '@chakra-ui/core';
import React from 'react';
import propTypes from 'prop-types';

const Footer: React.FC = ({ children }) => (
  <Flex direction="row" as="footer" py="1rem" align="flex-start">
    {children}
  </Flex>
);

Footer.propTypes = {
  children: propTypes.object,
};

export default Footer;
