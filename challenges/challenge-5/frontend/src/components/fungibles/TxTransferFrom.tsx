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

export default function TxTransferFrom(props: Props) {
  const { contract } = useContract<FungiblesContractApi>(ContractId.FUNGIBLES);
  const transferFromTx = useContractTx(contract, 'transferFrom');
  // Transaction parameters
  const [assetId, setAssetId] = useState<number | undefined>(undefined);
  const [fromAccount, setFromAccount] = useState<AccountId32Like | undefined>(undefined);
  const [toAccount, setToAccount] = useState<AccountId32Like | undefined>(undefined);
  const [amount, setAmount] = useState<bigint | undefined>(undefined);
  const isDisabled = useMemo(
    () => !assetId || !fromAccount || !toAccount || !amount,
    [assetId, fromAccount, toAccount, amount],
  );

  const handleSubmit = async () => {
    if (!assetId || !fromAccount || !toAccount || !amount) return;
    const toaster = txToaster('Signing transaction...');
    try {
      await transferFromTx.signAndSend({
        args: [assetId, fromAccount, toAccount, amount, ''],
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
      <ContractTxBadge title='transfer_from' />
      <Input
        type='number'
        mx={2}
        value={assetId}
        placeholder={'Asset ID'}
        maxWidth={150}
        onChange={(e) => setAssetId(parseInt(e.target.value))}
        isDisabled={transferFromTx.isInProgress}
      />
      <Input
        type='input'
        mx={2}
        value={fromAccount?.toString()}
        placeholder={'From Account (Starts with: 0x)'}
        onChange={(e) => setFromAccount(e.target.value)}
        isDisabled={transferFromTx.isInProgress}
      />
      <Input
        type='input'
        mx={2}
        value={toAccount?.toString()}
        placeholder={'To Account (Starts with: 0x)'}
        onChange={(e) => setToAccount(e.target.value)}
        isDisabled={transferFromTx.isInProgress}
      />
      <Input
        type='number'
        mx={2}
        value={amount ? amount.toString() : 0}
        placeholder={'Amount'}
        maxWidth={150}
        onChange={(e) => setAmount(BigInt(e.target.value))}
        isDisabled={transferFromTx.isInProgress}
      />
      <Button
        paddingX={8}
        size='md'
        isDisabled={isDisabled}
        isLoading={transferFromTx.isInProgress}
        onClick={handleSubmit}>
        Submit
      </Button>
    </Flex>
  );
}
