import { Button, Flex, Input } from '@chakra-ui/react';
import React, { useMemo, useState } from 'react';
import { ContractId } from '@/contracts/deployments';
import { FungiblesContractApi } from '@/contracts/types/fungibles';
import useContract from '@/hooks/useContract';
import useContractTx from '@/hooks/useContractTx';
import { txToaster } from '@/utils/txToaster';
import { AccountId32Like } from 'dedot/codecs';
import ContractTxBadge from '../shared/ContractTxBadge';

type Props = {};

export default function TxIncreaseAllowance(props: Props) {
  const { contract } = useContract<FungiblesContractApi>(ContractId.FUNGIBLES);
  const increaseAllowanceTx = useContractTx(contract, 'increaseAllowance');
  // Transaction parameters
  const [assetId, setAssetId] = useState<number | undefined>(undefined);
  const [spender, setSpender] = useState<AccountId32Like | undefined>(undefined);
  const [amount, setAmount] = useState<bigint | undefined>(undefined);
  const isDisabled = useMemo(() => !assetId || !spender || !amount, [assetId, spender, amount]);

  const handleSubmit = async () => {
    if (!assetId || !spender || !amount) return;
    const toaster = txToaster('Signing transaction...');
    try {
      await increaseAllowanceTx.signAndSend({
        args: [assetId, spender, amount],
        callback: ({ status }) => {
          console.log(status);
          toaster.updateTxStatus(status);
        },
      });
    } catch (e: any) {
      console.error(e);
      toaster.onError(e);
    }
  };
  return (
    <Flex alignItems={'center'}>
      <ContractTxBadge title='increase_allowance' />
      <Input
        type='number'
        mx={2}
        value={assetId}
        placeholder={'Asset ID'}
        maxWidth={150}
        onChange={(e) => setAssetId(parseInt(e.target.value))}
        isDisabled={increaseAllowanceTx.isInProgress}
      />
      <Input
        type='input'
        mx={2}
        value={spender?.toString()}
        placeholder={'Spender Address (Starts with: 0x)'}
        onChange={(e) => setSpender(e.target.value)}
        isDisabled={increaseAllowanceTx.isInProgress}
      />
      <Input
        type='number'
        mx={2}
        value={amount ? amount.toString() : 0}
        placeholder={'Amount'}
        maxWidth={150}
        onChange={(e) => setAmount(BigInt(e.target.value))}
        isDisabled={increaseAllowanceTx.isInProgress}
      />
      <Button
        paddingX={8}
        size='md'
        isDisabled={isDisabled}
        isLoading={increaseAllowanceTx.isInProgress}
        onClick={handleSubmit}>
        Submit
      </Button>
    </Flex>
  );
}
