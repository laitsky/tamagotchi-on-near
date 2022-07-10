import { useState, useEffect } from 'react';
import {
  Box,
  Button,
  Heading,
  Divider,
  Image,
} from '@chakra-ui/react';
import Navbar from './Navbar';

import Play from './Play';
import MintNFT from './MintNFT';

export default function Game() {
  const [hasUserMinted, setHasUserMinted] = useState(false);
  useEffect(() => {
    fetchData();
  }, []);


  const fetchData = async () => {
    const res = await window.contract.check_user_exists({ address: window.accountID });
    console.log(res);
    setHasUserMinted(res);
  }

  return (
    <Box>
      <Navbar />
      <Divider mt={3} mb={6} />
      {hasUserMinted ? <Play /> : <MintNFT />}
    </Box>
  )
}