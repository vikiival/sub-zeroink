import { Flex, Input } from '@chakra-ui/react';
import React, { useState } from 'react';
import { ContractId } from '@/contracts/deployments';
import { FungiblesContractApi } from '@/contracts/types/fungibles';
import useContract from '@/hooks/useContract';
import useContractQuery from '@/hooks/useContractQuery';
import ContractQueryBadge from '../shared/ContractQueryBadge';

type Props = {};

export default function QueryTokenDecimals(props: Props) {
  const { contract } = useContract<FungiblesContractApi>(ContractId.FUNGIBLES);
  const [assetId, setAssetId] = useState<number>(0);
  const query = useContractQuery({ contract, args: [assetId || 0], fn: 'tokenDecimals' });

  return (
    <Flex alignItems={'center'}>
      <ContractQueryBadge title='Total Name' />
      <Input
        type='number'
        mx={2}
        value={assetId}
        placeholder={'Asset ID'}
        maxWidth={150}
        onChange={(e) => setAssetId(parseInt(e.target.value))}
      />
      {query.data?.isOk ? query.data.value.toString() : query.isLoading ? 'Loading...' : 'Unknown'}
    </Flex>
  );
}
