import { createContext, useContext } from 'react';
import ClientProvider, {
  ClientContextProps,
  ClientProviderProps,
  useClientContext,
} from '@/providers/ClientProvider.tsx';
import WalletProvider, { useWalletContext, WalletContextProps } from '@/providers/WalletProvider.tsx';
import { ContractDeployment } from '@/types';

interface TypinkContextProps extends ClientContextProps, WalletContextProps {
  deployments: ContractDeployment[];
  defaultCaller: string;
}

export const TypinkContext = createContext<TypinkContextProps>({} as any);

export const useTypink = () => {
  return useContext(TypinkContext);
};

export interface TypinkProviderProps extends ClientProviderProps {
  deployments: ContractDeployment[];
  defaultCaller: string;
}

function TypinkProviderWrapper({ children, deployments, defaultCaller }: TypinkProviderProps) {
  const clientContext = useClientContext();
  const walletContext = useWalletContext();

  return (
    <TypinkContext.Provider value={{ ...clientContext, ...walletContext, deployments, defaultCaller }}>
      {children}
    </TypinkContext.Provider>
  );
}

export default function TypinkProvider({
  children,
  deployments,
  defaultCaller,
  defaultNetworkId,
}: TypinkProviderProps) {
  return (
    <WalletProvider>
      <ClientProvider defaultNetworkId={defaultNetworkId}>
        <TypinkProviderWrapper deployments={deployments} defaultCaller={defaultCaller}>
          {children}
        </TypinkProviderWrapper>
      </ClientProvider>
    </WalletProvider>
  );
}
