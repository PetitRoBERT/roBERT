import { Flex, Text } from '@chakra-ui/core';
import Body from './components/Body';
import Books from './components/AsyncComponent';
import DarkSwitch from './components/DarkSwitch';
import Emoji from './components/Emoji';
import Footer from './components/Footer';
import Heart from './components/Heart';
import Hero from './components/Hero';
import React from 'react';

const App = () => {
  return (
    <Body>
      <div style={{ textAlign: 'right' }}>
        <DarkSwitch />
      </div>
      <Hero>
        <span>
          Welcome to PetitRoBERT.
          <div style={{ display: 'inline-block', marginLeft: 50 }}>
            <span role="img" aria-label="warning">
              🚧‍
            </span>
            <Emoji />
            <span role="img" aria-label="warning">
              🚧
            </span>
          </div>
        </span>
      </Hero>
      <Books />
      <Footer>
        <Text fontSize={20}>Made with</Text>
        <Flex mx={2}>
          <Heart height={20} />
        </Flex>
        <Text fontSize={20}>by PetitRoBERT team.</Text>
      </Footer>
    </Body>
  );
};

export default App;
