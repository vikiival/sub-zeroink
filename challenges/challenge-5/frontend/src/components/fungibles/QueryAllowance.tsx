import { Flex, Input } from '@chakra-ui/react';
import React, { useState } from 'react';
import { ContractId } from '@/contracts/deployments';
import { FungiblesContractApi } from '@/contracts/types/fungibles';
import useContract from '@/hooks/useContract';
import useContractQuery from '@/hooks/useContractQuery';
import { AccountId32Like } from 'dedot/codecs';
import ContractQueryBadge from '../shared/ContractQueryBadge';

type Props = {};

export default function QueryAllowance(props: Props) {
  const { contract } = useContract<FungiblesContractApi>(ContractId.FUNGIBLES);
  const [assetId, setAssetId] = useState<number>(0);
  const [owner, setOwner] = useState<AccountId32Like | undefined>(undefined);
  const [spender, setSpender] = useState<AccountId32Like | undefined>(undefined);
  const query = useContractQuery({
    contract,
    args: [assetId || 0, owner || '0x', spender || '0x'],
    fn: 'allowance',
  });

  return (
    <Flex alignItems={'center'}>
      <ContractQueryBadge title='Alowance' />
      <Input
        type='number'
        mx={2}
        value={assetId}
        placeholder={'Asset ID'}
        maxWidth={150}
        onChange={(e) => setAssetId(parseInt(e.target.value))}
      />
      <Input
        type='input'
        mx={2}
        value={owner?.toString()}
        placeholder={'Owner address (Starts with: 0x)'}
        onChange={(e) => setOwner(e.target.value)}
      />
      <Input
        type='input'
        mx={2}
        value={spender?.toString()}
        placeholder={'Spender address (Starts with: 0x)'}
        onChange={(e) => setSpender(e.target.value)}
      />
      {query.data?.isOk ? query.data.value.toString() : query.isLoading ? 'Loading...' : 'Unknown'}
    </Flex>
  );
}
