import fungiblesMetadata from '@/contracts/fungibles/fungibles.json';
import { ContractDeployment } from '@/types.ts';
import { NetworkId } from '@/utils/networks.ts';

export enum ContractId {
  FUNGIBLES = 'fungibles',
}

export const fungibleContractDeployments: ContractDeployment[] = [
  {
    id: ContractId.FUNGIBLES,
    metadata: fungiblesMetadata as any,
    network: NetworkId.POP_TESTNET,
    address: '5GSGWox1ZxUkHBAEbm6NPAHLKD28VoQefTRBYTQuydLrxaKJ',
  },
  {
    id: ContractId.FUNGIBLES,
    metadata: fungiblesMetadata as any,
    network: NetworkId.ALEPHZERO_TESTNET,
    address: '5G5moUCkx5E2TD3CcRWvweg7rpCLngRmwukuKdaohvfBBmXr',
  },
];

export const deployments = [...fungibleContractDeployments];
