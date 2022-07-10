import { useState, useEffect } from 'react';
import {
  Box,
  Button,
  Heading,
  Image,
} from '@chakra-ui/react';

import DrumstickAsset from '../assets/drumstick.png';
import HappyAsset from '../assets/happy.webp';

export default function Play() {
  const [tamagotchi, setTamagotchi] = useState({});
  
  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    const res = await window.contract.get_user_tamagotchi({ address: window.accountID });
    console.log(res);
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

  const renderHungryMeter = (hungryMeter) => {
    if (hungryMeter > 0) {
      return Array.from({length: hungryMeter}, (_, i) => (
        <Image key={i} boxSize="40px" src={DrumstickAsset} />
      ))
    }
  }

  const renderHappinessMeter = (happinessMeter) => {
    if (happinessMeter > 0) {
      return Array.from({length: happinessMeter}, (_, i) => (
        <Image key={i} boxSize="30px" src={HappyAsset} background="transparent" />
      ))
    }
  }
  return (
    <Box display="flex" flexDirection="column">
      <Heading size="md">Weight: {tamagotchi.weight} kg</Heading>
      <Heading size="md">
        <Box display="flex" flexDirection="row" alignItems="center">
          Hungry: {renderHungryMeter(tamagotchi.hungry_meter)}
        </Box>
      </Heading>
      <Heading size="md">
        <Box display="flex" flexDirection="row" alignItems="center">
          Happiness: {renderHappinessMeter(tamagotchi.happiness_meter)}
        </Box>
      </Heading>
      <Box display="flex" flexDirection="row">
        <Button onClick={feed("MEAL")}>Meal</Button>
        <Button onClick={feed("SNACK")}>Snack</Button>
        <Button onClick={play("LEFT")}>Left</Button>
        <Button onClick={play("RIGHT")}>Right</Button>
        {tamagotchi.is_sick && <Button>Cure</Button> }
      </Box>
    </Box>
  )
}