import { Box, Button, Heading } from "@chakra-ui/react";

export default function MintNFT() {
  const mintNFT = async () => {
    await window.contract.tamagotchi_mint(
      {
        token_id: `tamagotchi.${window.accountID}`,
        metadata: {
          title: "Tamagotchi",
          description: "NEP-171 based NFT for your tamagotchi",
          media: "https://static.wikia.nocookie.net/tamagotchi/images/d/d3/Mametchi_blue.PNG/revision/latest?cb=20111002004702"
        },
        receiver_id: window.accountID
      },      
      300000000000000, // attached GAS (optional)
    )
  }

  return (
    <Box display="flex" flexDirection="column">
      <Heading>You have no NFT</Heading>
      <Heading size="md">Mint below to play the game</Heading>
      <Button onClick={mintNFT}>Mint NFT</Button>
    </Box>
  )
}