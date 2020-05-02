import App from './App';
import React from 'react';
import { render } from '@testing-library/react';

test('renders learn react link', () => {
  const { getByText } = render(<App />);
  const welcomeElement = getByText(/Welcome to PetitRoBERT./i);
  expect(welcomeElement).toBeInTheDocument();
});
