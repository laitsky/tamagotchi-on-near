import React from 'react';
import {
  ChakraProvider,
  Container,
} from '@chakra-ui/react';
import Game from './components/Game';
import RenderNotConnected from './components/RenderNotConnected';

import customTheme from './theme';

function App() {
  return (
    <ChakraProvider theme={customTheme}>
      <Container maxW="4xl" pt={4} style={{fontFamily: 'Press Start 2P'}}>
        {window.accountID ? <Game /> : <RenderNotConnected />}
      </Container>
    </ChakraProvider>
  );
}

export default App;
