import { Box, Heading } from "@chakra-ui/react";

export default function HowToPlay() {
  return (
    <>
      <Heading size="lg" mb={4}>How to Play</Heading>
      <Box mt={6}>
        <ol>
          <li>
            There are two ways to feed: "MEAL" and "SNACK". The player can give food
            to tamagotchi. Meal fills hungry hearts and adds 1 to the weight.
            Snack fills one hungry heart, adds 2 to the weight.
          </li>
          <li>
            You can play with the tamagotchi. Mechanics are simple, player just need
            to guess where the tamagotchi will be facing -- either left or right.
            Playing the game will reduce its weight and hungry meter by 1, and winning the game
            (taking the correct guess) will increase tamagotchi's heart by 1.
          </li>
          <li>
            If by somehow the tamagotchi is sick, giving the medicine will cure it. Sick tamagotchi 
            cannot be fed or played.
          </li>
          <li>
            Tamagotchi will sick if tamagotchi is overfeed. In the condition of overfeed,
            tamagotchi will sick and its happiness meter would decrease by 3.
          </li>
        </ol>
      </Box>
    </>
  )
}