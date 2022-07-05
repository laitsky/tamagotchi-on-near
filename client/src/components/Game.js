import { useState, useEffect } from 'react';
import {
  Box,
  Button,
  Heading,
  Divider,
  Image,
} from '@chakra-ui/react';

import Navbar from './Navbar';

import DrumstickAsset from '../assets/drumstick.png';
import HappyAsset from '../assets/happy.webp';

export default function Game() {
  const [state, setState] = useState({});

  useEffect(() => {
    getData();
  }, []);


  const getData = async () => {
    const res = await window.contract.get_state();
    setState(res);
    console.log("is sick?", res.is_sick)
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
    <Box>
      <Navbar />
      <Divider mt={3} mb={6} />
      <Box display="flex" flexDirection="column">
        <Heading size="md">Weight: {state.weight} kg</Heading>
        <Heading size="md">
          <Box display="flex" flexDirection="row" alignItems="center">
            Hungry: {renderHungryMeter(state.hungry_meter)}
          </Box>
        </Heading>
        <Heading size="md">
          <Box display="flex" flexDirection="row" alignItems="center">
            Happiness: {renderHappinessMeter(state.happiness_meter)}
          </Box>
        </Heading>
        <Box display="flex" flexDirection="row">
          <Button onClick={feed("MEAL")}>Meal</Button>
          <Button onClick={feed("SNACK")}>Snack</Button>
          <Button onClick={play("LEFT")}>Left</Button>
          <Button onClick={play("RIGHT")}>Right</Button>
          {state.is_sick && <Button>Cure</Button>}
        </Box>
      </Box>
    </Box>
  )
}