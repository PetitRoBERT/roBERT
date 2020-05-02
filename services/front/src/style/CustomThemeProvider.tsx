import { CSSReset, ColorModeProvider, ThemeProvider } from '@chakra-ui/core';
import React, { FunctionComponent } from 'react';
import propTypes from 'prop-types';

// This file is used to modify and apply the global Chakra UI theme.

const CustomThemeProvider: FunctionComponent = props => (
  <ThemeProvider>
    <CSSReset />
    <ColorModeProvider>{props.children}</ColorModeProvider>
  </ThemeProvider>
);

CustomThemeProvider.propTypes = {
  children: propTypes.object.isRequired,
};

export default CustomThemeProvider;
