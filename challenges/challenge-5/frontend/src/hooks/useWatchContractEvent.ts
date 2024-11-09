import { useEffect } from 'react';
import { useTypink } from '@/providers/TypinkProvider.tsx';
import { OmitNever } from '@/types.ts';
import { Contract, GenericContractApi } from 'dedot/contracts';
import { Unsub } from 'dedot/types';

type UseContractEvent<A extends GenericContractApi = GenericContractApi> = OmitNever<{
  [K in keyof A['events']]: K extends string ? (K extends `${infer Literal}` ? Literal : never) : never;
}>;

export default function useWatchContractEvent<
  T extends GenericContractApi = GenericContractApi,
  M extends keyof UseContractEvent<T> = keyof UseContractEvent<T>,
>(
  contract: Contract<T> | undefined,
  event: M,
  // remember to use `useCallback` for this callback :)
  onNewEvent: (events: ReturnType<T['events'][M]['filter']>) => void,
): void {
  const { client } = useTypink();

  useEffect(() => {
    if (!client || !contract) return;

    // handle unsubscribing when component unmounts
    let done = false;
    let unsub: Unsub | undefined;

    (async () => {
      unsub = await client.query.system.events((events) => {
        if (done) {
          unsub && unsub();
          return;
        }

        const contractEvents = contract.events[event].filter(events);
        if (contractEvents.length === 0) return;

        // @ts-ignore
        onNewEvent(contractEvents);
      });
    })();

    return () => {
      unsub && unsub();
      done = true;
    };
  }, [client, contract, onNewEvent]);
}
