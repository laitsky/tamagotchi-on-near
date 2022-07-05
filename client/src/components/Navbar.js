import {
  Box,
  Button,
  Heading,
  Menu,
  MenuButton,
  MenuList,
  MenuItem
} from '@chakra-ui/react';
import { ChevronDownIcon } from '@chakra-ui/icons';
import { logout } from '../utils';

export default function Navbar() {
  return (
    <Box display="flex" justifyContent="space-between">
      <Heading>Tamagotchi</Heading>
      <Menu>
        <MenuButton color="gray.700" as={Button} rightIcon={<ChevronDownIcon />} fontSize={12}>
          {window.accountID}
        </MenuButton>
        <MenuList color="gray.700">
          <MenuItem onClick={logout}>Logout</MenuItem>
        </MenuList>
      </Menu>
    </Box>
  )
}