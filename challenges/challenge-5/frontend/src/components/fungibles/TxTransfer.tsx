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

export default function TxTransfer(props: Props) {
  const { contract } = useContract<FungiblesContractApi>(ContractId.FUNGIBLES);
  const transferTx = useContractTx(contract, 'transfer');
  // Transaction parameters
  const [assetId, setAssetId] = useState<number | undefined>(undefined);
  const [account, setAccount] = useState<AccountId32Like | undefined>(undefined);
  const [amount, setAmount] = useState<bigint | undefined>(undefined);
  const isDisabled = useMemo(() => !assetId || !account || !amount, [assetId, account, amount]);

  const handleSubmit = async () => {
    if (!assetId || !account || !amount) return;
    const toaster = txToaster('Signing transaction...');
    try {
      await transferTx.signAndSend({
        args: [assetId, account, amount],
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
      <ContractTxBadge title='Transfer' />
      <Input
        type='number'
        mx={2}
        value={assetId}
        placeholder={'Asset ID'}
        maxWidth={150}
        onChange={(e) => setAssetId(parseInt(e.target.value))}
        isDisabled={transferTx.isInProgress}
      />
      <Input
        type='input'
        mx={2}
        value={account?.toString()}
        placeholder={'Recipient Account (Starts with: 0x)'}
        onChange={(e) => setAccount(e.target.value)}
        isDisabled={transferTx.isInProgress}
      />
      <Input
        type='number'
        mx={2}
        value={amount ? amount.toString() : 0}
        placeholder={'Amount'}
        maxWidth={150}
        onChange={(e) => setAmount(BigInt(e.target.value))}
        isDisabled={transferTx.isInProgress}
      />
      <Button paddingX={8} size='md' isDisabled={isDisabled} isLoading={transferTx.isInProgress} onClick={handleSubmit}>
        Submit
      </Button>
    </Flex>
  );
}
