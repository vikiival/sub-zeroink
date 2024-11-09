import { Box, Button, Flex, Menu, MenuButton, MenuDivider, MenuItem, MenuList, Text } from '@chakra-ui/react';
import { useEffect, useMemo } from 'react';
import ConnectedWallet from '@/components/dialog/ConnectedWallet.tsx';
import WalletSelection, { ButtonStyle } from '@/components/dialog/WalletSelection.tsx';
import useBalances from '@/hooks/useBalances.ts';
import useDisplayAddress from '@/hooks/useDisplayAddress';
import { useTypink } from '@/providers/TypinkProvider.tsx';
import { formatBalance, shortenAddress } from '@/utils/string.ts';

export default function AccountSelection() {
  const { accounts, selectedAccount, setSelectedAccount, signOut } = useTypink();
  const { network } = useTypink();
  const addresses = useMemo(() => accounts.map((a) => a.address), [accounts]);
  const balances = useBalances(addresses);

  const displayAddress = useDisplayAddress(selectedAccount?.address);

  useEffect(() => {
    if (selectedAccount && accounts.map((one) => one.address).includes(selectedAccount.address)) {
      return;
    }

    setSelectedAccount(accounts[0]);
  }, [accounts]);

  if (!selectedAccount) {
    return <></>;
  }

  const { name, address } = selectedAccount;

  return (
    <Box>
      <Menu autoSelect={false}>
        <MenuButton as={Button} variant='outline'>
          <Flex align='center' gap={2}>
            <Text fontWeight='semi-bold' fontSize='md'>
              {name}
            </Text>
            <Text fontSize='sm' fontWeight='400'>
              ({shortenAddress(displayAddress)})
            </Text>
          </Flex>
        </MenuButton>

        <MenuList>
          <ConnectedWallet />

          {accounts.map((one) => (
            <MenuItem
              backgroundColor={one.address === address ? 'gray.200' : ''}
              gap={2}
              key={one.address}
              onClick={() => setSelectedAccount(one)}>
              <Flex direction='column'>
                <Text fontWeight='500'>{one.name}</Text>

                <Text fontSize='xs'>Address: {shortenAddress(one.address)}</Text>
                <Text fontSize='xs'>
                  Balance: {formatBalance(balances[one.address]?.free) || '0'} {network.symbol}
                </Text>
              </Flex>
            </MenuItem>
          ))}
          <MenuDivider />
          <WalletSelection
            buttonStyle={ButtonStyle.MENU_ITEM}
            buttonLabel='Switch Wallet'
            buttonProps={{ color: 'primary.500' }}
          />
          <MenuItem onClick={signOut} color='red.500'>
            Sign Out
          </MenuItem>
        </MenuList>
      </Menu>
    </Box>
  );
}
