import { Badge } from '@chakra-ui/react';
import React from 'react';

type Props = {
  title: string;
};

export default function ContractQueryBadge({ title }: Props) {
  return (
    <Badge bg={'green'} color={'white'} mr={3}>
      {title}
    </Badge>
  );
}
