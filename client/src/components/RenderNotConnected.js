import {
  Center,
  Heading,
  Button
} from '@chakra-ui/react';
import { login } from '../utils';

export default function RenderNotConnected() {
  return (
    <Center display="flex" flexDirection="column">
      <Heading>Tamagotchi</Heading>
      <Button colorScheme="blue" onClick={login}>Login with NEAR</Button>
    </Center>
  )
}