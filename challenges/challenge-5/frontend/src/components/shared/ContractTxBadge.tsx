import { Badge } from '@chakra-ui/react';
import React from 'react';

type Props = {
  title: string;
};

export default function ContractTxBadge({ title }: Props) {
  return (
    <Badge bg={'pink'} mr={3}>
      {title}
    </Badge>
  );
}
