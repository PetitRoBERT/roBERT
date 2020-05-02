export const getBooks = async () =>
  new Promise<string[]>(resolve => {
    const _getBooks = () => resolve(['Harry Potter 1', 'Harry Potter 2', 'Harry Potter 3']);
    setTimeout(_getBooks, 10000);
  });
