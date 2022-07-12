import { useState, useEffect } from 'react';
import {
  Box,
  Button,
  Center,
  Heading,
  HStack,
  Image,
} from '@chakra-ui/react';

import HowToPlay from './HowToPlay';

import Mametchi from '../assets/mametchi.png';
import DrumstickAsset from '../assets/drumstick.png';
import HappyAsset from '../assets/happy.webp';

import '../index.css';

export default function Play() {
  const [tamagotchi, setTamagotchi] = useState({});
  
  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    const res = await window.contract.get_user_tamagotchi({ address: window.accountID });
    setTamagotchi(res);
  }
  const feed = (foodType) => async () => {
    try {
      await window.contract.feed({ food_type: foodType });
    } catch (err) {
      console.error(err);
    }
  }

  const play = (guess) => async () => {
    try {
      await window.contract.play({ guess });
    } catch (err) {
      console.error(err);
    }
  }

  const cure = async () => {
    try {
      await window.contract.cure();
    } catch (err) {
      console.error(err);
    }
  }

  const renderHungryMeter = (hungryMeter) => {
    if (hungryMeter > 0) {
      return Array.from({ length: hungryMeter }, (_, i) => (
        <Image key={i} boxSize="40px" src={DrumstickAsset} />
      ));
    }
  }

  const renderHappinessMeter = (happinessMeter) => {
    if (happinessMeter > 0) {
      return Array.from({ length: happinessMeter }, (_, i) => (
        <Image key={i} boxSize="30px" src={HappyAsset} background="transparent" />
      ));
    }
  }
  return (
    <Box display="flex" flexDirection="column" pb={8}>
      <Heading size="md">Weight: {tamagotchi.weight} kg</Heading>
      <Heading size="md">
        <Box display="flex" flexDirection="row" alignItems="center">
          Hungry: {renderHungryMeter(tamagotchi.hungry_meter)}
          {tamagotchi.overfeeding_meter >= 5 && <Heading size="sm" color="red.400">Overfeed</Heading>}
        </Box>
      </Heading>
      <Heading size="md">
        <Box display="flex" flexDirection="row" alignItems="center">
          Happiness: {renderHappinessMeter(tamagotchi.happiness_meter)}
        </Box>
      </Heading>
      <Center display="flex" flexDir="column">
      {tamagotchi.is_sick && <Heading size="md" color="red.400" className='blink'>Tamagotchi is sick!</Heading>}
      <Image 
          src={Mametchi} 
          alt="Mametchi"
          boxSize="300px"
          objectFit="cover"
          mt={8}
          mb={12}
        />
        <Box display="flex" flexDirection="row" mb={8}>
          {
            tamagotchi.is_sick ? 
            <Button onClick={cure}>Cure</Button> :
            (
              <HStack spacing="1.2em">
                <Button onClick={feed("MEAL")}>Meal</Button>
                <Button onClick={feed("SNACK")}>Snack</Button>
                <Button onClick={play("LEFT")}>Left</Button>
                <Button onClick={play("RIGHT")}>Right</Button>
              </HStack>
            )
          }
        </Box>
        <Box mt={10} />
        <HowToPlay />
      </Center>
    </Box>
  )
}