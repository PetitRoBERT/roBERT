import { List, ListItem, Spinner } from '@chakra-ui/core';
import React from 'react';
import { getBooks } from '../shared/books';
import { useAsync } from 'react-async';

const Books: React.FC = () => {
  const { data } = useAsync({ promiseFn: getBooks });

  return data !== undefined ? (
    <List>
      {data.map((d, i) => (
        <ListItem key={i}>{d}</ListItem>
      ))}
    </List>
  ) : (
    <Spinner />
  );
};

export default Books;
