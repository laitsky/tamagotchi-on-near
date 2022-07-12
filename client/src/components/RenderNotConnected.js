import {
  Box,
  Center,
  Heading,
  Button
} from '@chakra-ui/react';
import { login } from '../utils';

export default function RenderNotConnected() {
  return (
    <Box display="flex" justifyContent="center" minH="70vh">
      <Center display="flex" flexDirection="column">
        <Heading size="2xl" pb={12}>Tamagotchi</Heading>
        <Button colorScheme="blue" onClick={login} size="lg" mb={12}>Login with NEAR</Button>
        <Heading size="md" mb={4}>Made with 💙 by</Heading>
        <Heading size="md" textDecoration="underline" color="blue.200">
          <a href="https://github.com/laitsky" target="_blank" rel="noreferrer">laitsky</a>
        </Heading>
      </Center>
    </Box>
  )
}