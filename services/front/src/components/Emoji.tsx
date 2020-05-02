import React from 'react';
import { useColorMode } from '@chakra-ui/core';

const Emoji = () => {
  const { colorMode } = useColorMode();

  return colorMode === 'dark' ? (
    <span role="img" aria-label="constructor">
      👷🏻‍♂️
    </span>
  ) : (
    <span role="img" aria-label="constructor">
      👷🏿‍♂️
    </span>
  );
};

export default Emoji;
