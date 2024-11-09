import { Box, Divider, Heading, Stack } from '@chakra-ui/react';
import QueryAllowance from './QueryAllowance';
import QueryTokenDecimals from './QueryTokenDecimals';
import QueryTokenName from './QueryTokenName';
import QueryTokenSymbol from './QueryTokenSymbol';
import QueryTotalSupply from './QueryTotalSupply';
import TxApprove from './TxApprove';
import TxBurn from './TxBurn';
import TxDecreaseAllowance from './TxDecreaseAllowance';
import TxIncreaseAllowance from './TxIncreaseAllowance';
import TxMint from './TxMint';
import TxTransfer from './TxTransfer';
import TxTransferFrom from './TxTransferFrom';

export default function FungiblesBoard() {
  return (
    <Box>
      <Heading size='md'>POP API Fungibles Contract</Heading>
      <Stack mt={5}>
        <Heading size='sm'>Mutations</Heading>
        <TxMint />
        <TxBurn />
        <TxTransfer />
        <TxTransferFrom />
        <TxApprove />
        <TxIncreaseAllowance />
        <TxDecreaseAllowance />
        <Divider height={5} mb={5} />
        <Heading size='sm'>Queries</Heading>
        <QueryTotalSupply />
        <QueryAllowance />
        <QueryTokenName />
        <QueryTokenSymbol />
        <QueryTokenDecimals />
      </Stack>
    </Box>
  );
}
